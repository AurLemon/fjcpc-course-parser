<script setup>
import { ref, onMounted } from "vue";
import { useMotion } from "@vueuse/motion";
import { cn } from "./utils/cn";
import CourseTable from "./components/CourseTable.vue";

const ucode = ref("");
const loading = ref(false);
const scheduleData = ref(null);
const error = ref("");
const loadingProgress = ref(0);

// 从 localStorage 加载 ucode
onMounted(() => {
  const savedUcode = localStorage.getItem("fjcpc_ucode");
  if (savedUcode) {
    ucode.value = savedUcode;
  }
});

// 模拟加载进度
const simulateProgress = () => {
  loadingProgress.value = 0;
  const interval = setInterval(() => {
    if (loadingProgress.value < 90) {
      loadingProgress.value += Math.random() * 10;
    }
    if (!loading.value) {
      clearInterval(interval);
      loadingProgress.value = 100;
      setTimeout(() => {
        loadingProgress.value = 0;
      }, 500);
    }
  }, 500);
};

// 获取课表
const fetchSchedule = async () => {
  if (!ucode.value.trim()) {
    error.value = "请输入 UCode";
    return;
  }

  loading.value = true;
  error.value = "";
  scheduleData.value = null;
  simulateProgress();

  try {
    // 保存 ucode 到 localStorage
    localStorage.setItem("fjcpc_ucode", ucode.value.trim());

    const response = await fetch("http://127.0.0.1:4000/api/schedule", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ucode: ucode.value.trim(),
      }),
    });

    if (!response.ok) {
      throw new Error("获取课表失败");
    }

    const result = await response.json();

    if (result.status === "success") {
      scheduleData.value = result.data;
    } else {
      throw new Error(result.message || "获取课表失败");
    }
  } catch (err) {
    error.value = err.message || "网络错误，请稍后重试";
  } finally {
    loading.value = false;
  }
};

// 清空数据
const clearData = () => {
  scheduleData.value = null;
  error.value = "";
};
</script>

<template>
  <div class="min-h-screen bg-background">
    <div class="container mx-auto px-4 py-8 max-w-6xl">
      <!-- 标题 -->
      <div
        v-motion
        :initial="{ opacity: 0, y: -20 }"
        :enter="{
          opacity: 1,
          y: 0,
          transition: { duration: 600, ease: 'easeOut' },
        }"
        class="text-center mb-8"
      >
        <h1 class="text-3xl font-bold text-gray-900 mb-2">
          课表查询
        </h1>
        <p class="text-gray-600 text-sm">输入 UCode 查看课程表</p>
      </div>

      <!-- 输入区域 -->
      <div
        v-motion
        :initial="{ opacity: 0, y: 20 }"
        :enter="{
          opacity: 1,
          y: 0,
          transition: { duration: 600, delay: 100, ease: 'easeOut' },
        }"
        class="bg-white rounded-2xl shadow-sm border-[0.5px] border-gray-200 p-6 mb-6"
      >
        <div class="flex flex-col sm:flex-row gap-3">
          <input
            v-model="ucode"
            type="text"
            placeholder="请输入 UCode"
            :disabled="loading"
            @keyup.enter="fetchSchedule"
            :class="
              cn(
                'flex-1 px-4 py-3 rounded-xl border-[0.5px] border-gray-300 bg-white',
                'focus:outline-none focus:ring-1 focus:ring-gray-400 focus:border-gray-400',
                'transition-all duration-200',
                'disabled:opacity-50 disabled:cursor-not-allowed',
                'text-sm'
              )
            "
          />
          <button
            @click="fetchSchedule"
            :disabled="loading || !ucode.trim()"
            :class="
              cn(
                'px-6 py-3 rounded-xl font-medium text-sm',
                'bg-gray-900 text-white',
                'hover:bg-gray-800 active:scale-95',
                'transition-all duration-200',
                'disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100',
                'shadow-sm'
              )
            "
          >
            {{ loading ? "加载中..." : "查询课表" }}
          </button>
          <button
            v-if="scheduleData"
            @click="clearData"
            :disabled="loading"
            :class="
              cn(
                'px-6 py-3 rounded-xl font-medium text-sm',
                'bg-gray-100 text-gray-900',
                'hover:bg-gray-200 active:scale-95',
                'transition-all duration-200',
                'disabled:opacity-50 disabled:cursor-not-allowed',
                'shadow-sm'
              )
            "
          >
            清空
          </button>
        </div>

        <!-- 加载进度条 -->
        <div
          v-if="loading"
          v-motion
          :initial="{ opacity: 0, scaleX: 0 }"
          :enter="{ opacity: 1, scaleX: 1, transition: { duration: 300 } }"
          class="mt-4 h-2 bg-gray-100 rounded-full overflow-hidden"
        >
          <div
            class="h-full bg-gray-900 transition-all duration-500 ease-out rounded-full"
            :style="{ width: `${loadingProgress}%` }"
          />
        </div>

        <!-- 错误提示 -->
        <div
          v-if="error"
          v-motion
          :initial="{ opacity: 0, y: -10 }"
          :enter="{ opacity: 1, y: 0, transition: { duration: 300 } }"
          class="mt-4 p-3 bg-red-50 border-[0.5px] border-red-200 rounded-xl text-red-600 text-sm"
        >
          {{ error }}
        </div>
      </div>

      <!-- 课表展示 -->
      <CourseTable v-if="scheduleData" :schedule-data="scheduleData" />
    </div>
  </div>
</template>

<style scoped>
/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: hsl(var(--muted));
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.3);
}
</style>
