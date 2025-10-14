<script setup>
import { ref, computed } from 'vue'
import { cn } from '../utils/cn'

const props = defineProps({
  scheduleData: {
    type: Object,
    required: true
  }
})

const currentWeek = ref(1)

// 获取所有周数
const weeks = computed(() => {
  if (!props.scheduleData?.weeks) return []
  return Object.keys(props.scheduleData.weeks)
    .map(Number)
    .sort((a, b) => a - b)
})

// 当前周的课程数据
const currentWeekData = computed(() => {
  if (!props.scheduleData?.weeks) return []
  return props.scheduleData.weeks[currentWeek.value] || []
})

// 星期映射
const weekdayNames = ['', '周一', '周二', '周三', '周四', '周五', '周六', '周日']

// 课程节次时间映射
const courseTimeMap = {
  1: '08:00-08:45',
  2: '08:55-09:40',
  3: '10:00-10:45',
  4: '10:55-11:40',
  5: '14:00-14:45',
  6: '14:55-15:40',
  7: '16:00-16:45',
  8: '16:55-17:40',
  9: '19:00-19:45',
  10: '19:55-20:40',
  11: '20:50-21:35',
  12: '21:45-22:30',
}

// 切换周
const changeWeek = (week) => {
  currentWeek.value = week
}

// 获取课程卡片样式
const getCourseStyle = (course) => {
  return {
    backgroundColor: course.color || '#3b82f6',
    gridRow: `span ${course.continuous_course || 1}`,
  }
}
</script>

<template>
  <div 
    v-motion
    :initial="{ opacity: 0, y: 20 }"
    :enter="{ opacity: 1, y: 0, transition: { duration: 600, delay: 200, ease: 'easeOut' } }"
    class="bg-white rounded-2xl shadow-sm border-[0.5px] border-gray-200 overflow-hidden"
  >
    <!-- 周选择器 -->
    <div class="border-b-[0.5px] border-gray-200 p-4">
      <div class="flex items-center gap-2 overflow-x-auto pb-2">
        <span class="text-sm text-gray-600 whitespace-nowrap mr-2">选择周次:</span>
        <div class="flex gap-2">
          <button
            v-for="week in weeks"
            :key="week"
            @click="changeWeek(week)"
            :class="cn(
              'px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
              'hover:scale-105 active:scale-95 whitespace-nowrap',
              currentWeek === week
                ? 'bg-gray-900 text-white shadow-sm'
                : 'bg-gray-100 text-gray-900 hover:bg-gray-200'
            )"
          >
            第{{ week }}周
          </button>
        </div>
      </div>
    </div>

    <!-- 课表网格 -->
    <div class="p-4 overflow-x-auto">
      <div class="min-w-[800px]">
        <!-- 表头 -->
        <div class="grid grid-cols-8 gap-2 mb-3">
          <div class="text-center text-sm font-medium text-gray-600 py-2">
            节次
          </div>
          <div
            v-for="day in 7"
            :key="day"
            class="text-center text-sm font-medium text-gray-900 py-2"
          >
            {{ weekdayNames[day] }}
          </div>
        </div>

        <!-- 课程表格 -->
        <div class="space-y-2">
          <div
            v-for="courseNum in 12"
            :key="courseNum"
            class="grid grid-cols-8 gap-2"
          >
            <!-- 节次列 -->
            <div class="flex flex-col items-center justify-center bg-gray-100 rounded-lg p-2">
              <div class="text-xs font-medium text-gray-900">{{ courseNum }}</div>
              <div class="text-[10px] text-gray-600 mt-0.5">
                {{ courseTimeMap[courseNum] }}
              </div>
            </div>

            <!-- 每天的课程 -->
            <div
              v-for="day in 7"
              :key="day"
              class="min-h-[60px] rounded-lg border-[0.5px] border-gray-200 bg-gray-50/30"
            >
              <template v-for="dayCourse in currentWeekData" :key="dayCourse.weekday">
                <template v-if="dayCourse.weekday === day">
                  <template v-for="slot in dayCourse.course" :key="slot.course_number">
                    <div
                      v-if="slot.course_info && slot.course_number === courseNum"
                      v-motion
                      :initial="{ opacity: 0, scale: 0.9 }"
                      :enter="{ opacity: 1, scale: 1, transition: { duration: 300, delay: 50 } }"
                      :class="cn(
                        'p-2 rounded-lg text-white text-xs h-full',
                        'hover:shadow-lg hover:scale-[1.02] transition-all duration-200',
                        'cursor-pointer'
                      )"
                      :style="{ backgroundColor: slot.course_info.color }"
                    >
                      <div class="font-medium mb-1 line-clamp-2">
                        {{ slot.course_info.name }}
                      </div>
                      <div class="text-[10px] opacity-90 space-y-0.5">
                        <div v-if="slot.course_info.classroom" class="line-clamp-1">
                          📍 {{ slot.course_info.classroom }}
                        </div>
                        <div v-if="slot.course_info.teacher?.length" class="line-clamp-1">
                          👨‍🏫 {{ slot.course_info.teacher.join(', ') }}
                        </div>
                        <div v-if="slot.course_info.continuous_course > 1" class="line-clamp-1">
                          ⏱️ {{ slot.course_info.continuous_course }}节连上
                        </div>
                      </div>
                    </div>
                  </template>
                </template>
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-if="!currentWeekData || currentWeekData.length === 0"
      class="p-12 text-center text-gray-600"
    >
      <div class="text-4xl mb-2">📅</div>
      <div class="text-sm">本周暂无课程</div>
    </div>
  </div>
</template>

<style scoped>
/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: hsl(var(--muted));
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.3);
}

.line-clamp-1 {
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 1;
}

.line-clamp-2 {
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
}
</style>

