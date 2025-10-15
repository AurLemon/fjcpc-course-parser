<script setup>
import { ref } from 'vue'

const emit = defineEmits(['close'])

const currentPage = ref(1)
const totalPages = 3

const tutorials = [
  {
    title: '第一步：获取课表链接',
    content: `
      <ol class="list-decimal list-inside space-y-2 text-sm text-gray-700">
        <li>登录福建船政交通职业学院教务系统</li>
        <li>进入"我的课表"页面</li>
        <li>复制浏览器地址栏中的完整链接</li>
        <li>链接格式类似：https://asp.fjcpc.edu.cn/czmobile/mytimetableindex/New?uid=XXXXXX</li>
      </ol>
    `,
  },
  {
    title: '第二步：提取 UCode',
    content: `
      <ol class="list-decimal list-inside space-y-2 text-sm text-gray-700">
        <li>从链接中找到 <code class="px-1 py-0.5 bg-gray-100 rounded text-xs">uid=</code> 后面的部分</li>
        <li>这就是你的 UCode，例如：<code class="px-1 py-0.5 bg-gray-100 rounded text-xs">133****7573</code></li>
        <li>你可以直接粘贴完整链接，系统会自动提取 UCode</li>
      </ol>
    `,
  },
  {
    title: '第三步：查询课表',
    content: `
      <ol class="list-decimal list-inside space-y-2 text-sm text-gray-700">
        <li>在顶部输入框中粘贴 UCode 或完整链接</li>
        <li>点击搜索按钮或按回车键</li>
        <li>等待系统加载课表数据</li>
        <li>查看你的课程安排</li>
      </ol>
      <div class="mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg text-sm text-blue-800">
        💡 提示：系统会自动保存你的 UCode，下次访问时无需重新输入
      </div>
    `,
  },
]

const nextPage = () => {
  if (currentPage.value < totalPages) {
    currentPage.value++
  }
}

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

const close = () => {
  emit('close')
}
</script>

<template>
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
    @click.self="close"
  >
    <div
      class="bg-white rounded-2xl border-[0.5px] border-gray-200 max-w-2xl w-full max-h-[80vh] flex flex-col"
    >
      <!-- 头部 -->
      <div
        class="flex items-center justify-between p-6 border-b border-gray-200"
      >
        <h2 class="text-xl font-semibold text-gray-900">使用教程</h2>
        <button
          @click="close"
          class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-gray-100 text-gray-500 hover:text-gray-700"
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
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- 内容区 -->
      <div class="flex-1 overflow-y-auto p-6">
        <transition name="slide" mode="out-in">
          <div :key="currentPage" class="space-y-4">
            <h3 class="text-lg font-medium text-gray-900">
              {{ tutorials[currentPage - 1].title }}
            </h3>
            <div v-html="tutorials[currentPage - 1].content"></div>
          </div>
        </transition>
      </div>

      <!-- 底部分页 -->
      <div
        class="flex items-center justify-between p-6 border-t border-gray-200"
      >
        <button
          @click="prevPage"
          :disabled="currentPage === 1"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          上一步
        </button>

        <div class="flex items-center gap-2">
          <button
            v-for="page in totalPages"
            :key="page"
            @click="currentPage = page"
            :class="[
              'w-8 h-8 rounded-full text-sm font-medium transition-colors',
              currentPage === page
                ? 'bg-gray-900 text-white'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
            ]"
          >
            {{ page }}
          </button>
        </div>

        <button
          @click="nextPage"
          :disabled="currentPage === totalPages"
          class="px-4 py-2 text-sm font-medium text-white bg-gray-900 rounded-lg hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          下一步
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
}

.slide-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.slide-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}
</style>
