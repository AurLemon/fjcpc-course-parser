<script setup>
import { ref, onMounted, computed, watch } from 'vue';
import CourseTable from './components/CourseTable.vue';
import TutorialModal from './components/TutorialModal.vue';

const inputValue = ref('');
const loading = ref(false);
const scheduleData = ref(null);
const showTutorial = ref(false);
const toast = ref({ show: false, message: '', type: 'success' });

// 主题模式：'system' | 'light' | 'dark'
const themeMode = ref('system');

// 简单的时令提示（无需先请求课表）
const seasonHint = ref('');

// 高级选项：并行与缓存（默认并行+开启缓存；下拉默认收起）
const showAdvanced = ref(false);
const optParallel = ref(true);
const optUseCache = ref(true);

// 从 localStorage 加载 ucode 和主题，并并行拉取当前时令
onMounted(async () => {
  const savedUcode = localStorage.getItem('fjcpc_ucode');
  if (savedUcode) {
    inputValue.value = savedUcode;
  }

  // 读取高级选项本地存储
  const savedParallel = localStorage.getItem('fjcpc_parallel');
  if (savedParallel !== null) optParallel.value = savedParallel === '1';
  const savedUseCache = localStorage.getItem('fjcpc_use_cache');
  if (savedUseCache !== null) optUseCache.value = savedUseCache === '1';

  const savedTheme = localStorage.getItem('fjcpc_theme');
  if (savedTheme) {
    themeMode.value = savedTheme;
  }

  // 预取当前时令
  try {
    const r = await fetch('http://127.0.0.1:4000/api/season');
    if (r.ok) {
      const j = await r.json();
      if (j.status === 'success' && j.data?.season) {
        seasonHint.value = j.data.season;
      }
    }
  } catch (_) {
    // 静默失败，不影响主流程
  }
});

// 应用主题到 <html>（class = dark）
const applyTheme = () => {
  const root = document.documentElement;
  const prefersDark =
    window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: dark)').matches;
  const effective =
    themeMode.value === 'system'
      ? prefersDark
        ? 'dark'
        : 'light'
      : themeMode.value;
  if (effective === 'dark') root.classList.add('dark');
  else root.classList.remove('dark');
};

onMounted(() => {
  applyTheme();
  // 跟随系统变化
  const mq = window.matchMedia('(prefers-color-scheme: dark)');
  const onChange = () => {
    if (themeMode.value === 'system') applyTheme();
  };
  mq.addEventListener?.('change', onChange);
});

watch(themeMode, applyTheme);

// 切换主题

// 持久化高级选项
watch(optParallel, (v) =>
  localStorage.setItem('fjcpc_parallel', v ? '1' : '0')
);
watch(optUseCache, (v) =>
  localStorage.setItem('fjcpc_use_cache', v ? '1' : '0')
);

const cycleTheme = () => {
  const modes = ['system', 'light', 'dark'];
  const currentIndex = modes.indexOf(themeMode.value);
  const nextIndex = (currentIndex + 1) % modes.length;
  themeMode.value = modes[nextIndex];
  localStorage.setItem('fjcpc_theme', themeMode.value);
};

// 主题图标和文字
const themeIcon = computed(() => {
  if (themeMode.value === 'system')
    return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
  if (themeMode.value === 'light')
    return 'M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z';
  return 'M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z';
});

const themeText = computed(() => {
  if (themeMode.value === 'system') return '跟随系统';
  if (themeMode.value === 'light') return '浅色模式';
  return '深色模式';
});

// 显示提示消息
const showToast = (message, type = 'success') => {
  toast.value = { show: true, message, type };
  setTimeout(() => {
    toast.value.show = false;
  }, 3000);
};

// 提取 UCode
const extractUCode = (input) => {
  const trimmed = input.trim();
  // 如果是URL，提取UCode
  const urlMatch = trimmed.match(/[?&]uid=([^&]+)/i);
  if (urlMatch) {
    return urlMatch[1];
  }
  // 否则直接返回输入
  return trimmed;
};

// 获取课表
const fetchSchedule = async () => {
  const ucode = extractUCode(inputValue.value);

  if (!ucode) {
    showToast('请输入 UCode 或课表链接', 'error');
    return;
  }

  loading.value = true;
  scheduleData.value = null;

  try {
    // 保存原始输入到 localStorage
    localStorage.setItem('fjcpc_ucode', inputValue.value.trim());

    const response = await fetch('http://127.0.0.1:4000/api/schedule', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        ucode,
        parallel: optParallel.value,
        use_cache: optUseCache.value,
      }),
    });

    if (!response.ok) {
      throw new Error('获取课表失败');
    }

    const result = await response.json();

    if (result.status === 'success') {
      scheduleData.value = result.data;
      showToast('✓ 检测到课表数据，已还原成课表格式并打包回传。', 'success');
    } else {
      throw new Error(result.message || '获取课表失败');
    }
  } catch (err) {
    showToast(err.message || '网络错误，请稍后重试', 'error');
  } finally {
    loading.value = false;
  }
};

// 清空输入
const clearInput = () => {
  inputValue.value = '';
  scheduleData.value = null;
};

// 处理回车键
const handleKeyPress = (e) => {
  if (e.key === 'Enter') {
    fetchSchedule();
  }
};
</script>

<template>
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 via-gray-100 to-gray-50 dark:from-gray-950 dark:via-gray-950 dark:to-gray-950"
  >
    <!-- 顶部导航栏 -->
    <header
      v-motion
      :initial="{ y: -100, opacity: 0 }"
      :enter="{
        y: 0,
        opacity: 1,
        transition: { duration: 600, ease: 'easeOut' },
      }"
      class="bg-white/80 dark:bg-gray-900/70 backdrop-blur-xl border-b-[0.5px] border-gray-200 dark:border-gray-800 sticky top-0 z-50"
    >
      <div
        class="max-w-7xl mx-auto px-4 h-16 flex items-center justify-between"
      >
        <!-- 左侧：Logo + 标题 -->
        <div class="flex items-center gap-2.5">
          <svg
            class="w-6 h-6 text-gray-900"
            viewBox="0 0 16 16"
            fill="currentColor"
          >
            <path
              d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
            />
          </svg>
          <h1 class="text-lg font-semibold text-gray-900">
            FJCPC Course Parser
          </h1>
        </div>

        <!-- 中间：输入框 -->
        <div class="flex-1 max-w-xl mx-8">
          <div class="relative group">
            <input
              v-model="inputValue"
              type="text"
              placeholder="输入 UCode 或带 UCode 的课表链接即可获取你的课表"
              :disabled="loading"
              @keypress="handleKeyPress"
              class="w-full px-4 py-2.5 text-sm bg-gray-50/50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 border-[0.5px] border-gray-200 dark:border-gray-700 rounded-3xl focus:outline-none focus:ring-2 focus:ring-gray-900/20 focus:border-gray-900 dark:focus:border-gray-100 focus:bg-white dark:focus:bg-gray-900 disabled:opacity-50 transition-colors duration-200 group-hover:border-gray-300"
            />
            <button
              v-if="inputValue"
              @click="clearInput"
              class="absolute right-14 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition-colors duration-200"
            >
              <svg
                class="w-4 h-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>

            <button
              @click="fetchSchedule"
              :disabled="loading || !inputValue.trim()"
              class="absolute right-7 top-1/2 -translate-y-1/2 text-gray-600 hover:text-gray-900 disabled:opacity-30 disabled:cursor-not-allowed transition-colors duration-200"
            >
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
              </svg>
            </button>

            <!-- 高级开关按钮：箭头朝上，展开时旋转朝下 -->
            <button
              @click="showAdvanced = !showAdvanced"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-gray-100 transition-colors duration-200"
              :aria-expanded="showAdvanced ? 'true' : 'false'"
              title="高级选项"
            >
              <svg
                class="w-4 h-4 transition-transform duration-200"
                :class="showAdvanced ? 'rotate-180' : 'rotate-0'"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M5.23 12.21a.75.75 0 001.06.02L10 8.707l3.71 3.524a.75.75 0 001.04-1.082l-4.25-4.04a.75.75 0 00-1.04 0l-4.25 4.04a.75.75 0 00-.02 1.06z"
                  clip-rule="evenodd"
                />
              </svg>
            </button>

            <!-- 高级下拉菜单：带动画 -->
            <transition
              enter-active-class="transition transform duration-200"
              enter-from-class="opacity-0 -translate-y-1 scale-95"
              enter-to-class="opacity-100 translate-y-0 scale-100"
              leave-active-class="transition transform duration-150"
              leave-from-class="opacity-100 translate-y-0 scale-100"
              leave-to-class="opacity-0 -translate-y-1 scale-95"
            >
              <div
                v-if="showAdvanced"
                class="absolute right-0 top-full mt-2 w-64 p-3 bg-white dark:bg-gray-900 border-[0.5px] border-gray-200 dark:border-gray-800 rounded-xl shadow-sm"
              >
                <div class="flex items-center justify-between py-1.5">
                  <div>
                    <p class="text-sm text-gray-900 dark:text-gray-100">
                      并行抓取
                    </p>
                    <p class="text-xs text-gray-500">更快（默认开启）</p>
                  </div>
                  <input
                    type="checkbox"
                    v-model="optParallel"
                    class="h-4 w-4"
                  />
                </div>
                <div class="h-px bg-gray-200 dark:bg-gray-800 my-2"></div>
                <div class="flex items-center justify-between py-1.5">
                  <div>
                    <p class="text-sm text-gray-900 dark:text-gray-100">
                      使用缓存
                    </p>
                    <p class="text-xs text-gray-500">
                      命中直接返回（默认开启）
                    </p>
                  </div>
                  <input
                    type="checkbox"
                    v-model="optUseCache"
                    class="h-4 w-4"
                  />
                </div>
              </div>
            </transition>
          </div>
        </div>

        <!-- 右侧：主题切换 + 后端时令标签 + 帮助 -->
        <div class="flex items-center gap-4">
          <!-- 非交互：由后端决定的时令标签 -->
          <span
            v-if="scheduleData?.season || seasonHint"
            class="text-lg text-gray-600 dark:text-gray-300 select-none"
          >
            {{
              (scheduleData?.season || seasonHint) === 'winter'
                ? '冬令时作息'
                : (scheduleData?.season || seasonHint) === 'summer'
                ? '夏令时作息'
                : ''
            }}
          </span>

          <!-- 主题：跟随系统/浅色/深色 -->
          <button
            @click="cycleTheme"
            class="flex items-center text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100 transition-colors duration-200"
            :title="themeText"
            aria-label="toggle theme"
          >
            <svg
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                :d="themeIcon"
              />
            </svg>
          </button>

          <!-- 帮助 -->
          <button
            @click="showTutorial = true"
            class="text-gray-600 hover:text-gray-900 transition-colors duration-200"
            title="使用教程"
          >
            <svg
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </button>
        </div>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="max-w-7xl mx-auto px-4 py-6 h-[calc(100vh-4rem)]">
      <!-- Toast 提示 -->

      <transition
        enter-active-class="transition-all duration-300"
        leave-active-class="transition-all duration-200"
        enter-from-class="opacity-0 -translate-y-4"
        leave-to-class="opacity-0 -translate-y-4"
      >
        <div
          v-if="toast.show"
          v-motion
          :initial="{ opacity: 0, y: -20, scale: 0.95 }"
          :enter="{ opacity: 1, y: 0, scale: 1, transition: { duration: 300 } }"
          :class="[
            'fixed top-24 left-1/2 -translate-x-1/2 px-5 py-3 rounded-xl z-50 backdrop-blur-sm border-[0.5px]',
            toast.type === 'success'
              ? 'bg-green-50/90 text-green-900 border-green-200'
              : 'bg-red-50/90 text-red-900 border-red-200',
          ]"
        >
          <div class="flex items-center gap-2 text-sm font-medium">
            <svg
              v-if="toast.type === 'success'"
              class="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <svg
              v-else
              class="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>{{ toast.message }}</span>
          </div>
        </div>
      </transition>

      <!-- 加载状态 -->
      <div
        v-if="loading"
        v-motion
        :initial="{ opacity: 0, y: 20 }"
        :enter="{ opacity: 1, y: 0, transition: { duration: 500 } }"
        class="flex flex-col items-center justify-center py-40"
      >
        <div
          class="relative w-72 h-2 bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200 rounded-full overflow-hidden"
        >
          <div
            class="absolute inset-0 bg-gradient-to-r from-gray-900 via-gray-700 to-gray-900 animate-loading rounded-full"
          ></div>
        </div>
        <p class="mt-6 text-base font-medium text-gray-700">
          正在加载课表数据...
        </p>
        <p class="mt-2 text-sm text-gray-500">请稍候</p>
      </div>

      <!-- 课表展示 -->
      <CourseTable v-else-if="scheduleData" :schedule-data="scheduleData" />

      <!-- 空状态 -->
      <div
        v-else
        v-motion
        :initial="{ opacity: 0, y: 20 }"
        :enter="{ opacity: 1, y: 0, transition: { duration: 500 } }"
        class="flex flex-col items-center justify-center py-40"
      >
        <svg
          class="w-20 h-20 text-gray-400 mb-6"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="1.5"
            d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
          />
        </svg>
        <p class="text-2xl text-center text-gray-900 mb-2">
          欢迎使用 FJCPC Course Parser，<br />
          请在上方输入 UCode 或含有 UCode 的课表链接开始查询。
        </p>
        <button
          @click="showTutorial = true"
          class="mt-2 cursor-pointer text-gray-400 text-sm hover:bg-gray-200/50 px-3 py-2 rounded-md transition-all duration-200"
        >
          <span>如何获取我的 UCode</span>
        </button>
      </div>
    </main>

    <!-- 底部 -->
    <footer
      v-motion
      :initial="{ opacity: 0 }"
      :enter="{ opacity: 1, transition: { duration: 600, delay: 400 } }"
      class="border-t-[0.5px] border-gray-200 mt-20 bg-gray-50"
    >
      <div class="max-w-7xl mx-auto px-4 py-8 text-center">
        <div
          class="flex items-center justify-center gap-4 flex-wrap text-xs text-gray-600 mb-4"
        >
          <a href="#" class="hover:text-gray-900 transition-colors duration-200"
            >隐私政策</a
          >
          <span class="text-gray-300">·</span>
          <a href="#" class="hover:text-gray-900 transition-colors duration-200"
            >FAQ</a
          >
          <span class="text-gray-300">·</span>
          <button
            @click="showTutorial = true"
            class="hover:text-gray-900 transition-colors duration-200"
          >
            使用教程
          </button>
          <span class="text-gray-300">·</span>
          <a
            href="https://github.com"
            target="_blank"
            class="hover:text-gray-900 transition-colors duration-200 flex items-center gap-1"
          >
            <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 16 16">
              <path
                d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
              />
            </svg>
            GitHub Repository
          </a>
          <span class="text-gray-300">·</span>
          <span class="text-gray-500">MIT License</span>
        </div>
        <p class="text-xs text-gray-500 leading-relaxed max-w-2xl mx-auto">
          项目基于 Rust Actix + Vue 实现，仅供编程学习交流，请在 24
          小时后删除有关内容
        </p>
      </div>
    </footer>

    <!-- 教程弹窗 -->
    <TutorialModal v-if="showTutorial" @close="showTutorial = false" />
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, -20px);
}

@keyframes loading {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(400%);
  }
}

.animate-loading {
  animation: loading 1.5s ease-in-out infinite;
}
</style>
