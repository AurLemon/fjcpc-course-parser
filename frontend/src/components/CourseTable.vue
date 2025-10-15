<script setup>
import { ref, computed } from "vue";
import { cn } from "../utils/cn";

const props = defineProps({
  scheduleData: {
    type: Object,
    required: true,
  },
});

const currentWeek = ref(1);
const selectedCourse = ref(null);
const showDialog = ref(false);

// 获取所有周数
const weeks = computed(() => {
  if (!props.scheduleData?.weeks) return [];
  return Object.keys(props.scheduleData.weeks)
    .map(Number)
    .sort((a, b) => a - b);
});

// 当前周的课程数据
const currentWeekData = computed(() => {
  if (!props.scheduleData?.weeks) return [];
  return props.scheduleData.weeks[currentWeek.value] || [];
});

// 星期映射
const weekdayNames = ["", "周一", "周二", "周三", "周四", "周五", "周六", "周日"];

// 课程节次时间映射
const courseTimeMap = computed(() => {
  if (!props.scheduleData?.time_table) {
    // 默认时间表
    return {
      1: "08:00-08:45",
      2: "08:55-09:40",
      3: "10:00-10:45",
      4: "10:55-11:40",
      5: "14:00-14:45",
      6: "14:55-15:40",
      7: "16:00-16:45",
      8: "16:55-17:40",
      9: "19:00-19:45",
      10: "19:55-20:40",
    };
  }

  const map = {};
  props.scheduleData.time_table.forEach((time, index) => {
    map[index + 1] = `${time[0]}-${time[1]}`;
  });
  return map;
});

// 切换周
const changeWeek = (week) => {
  currentWeek.value = week;
};

// 打开课程详情
const openCourseDialog = (courseInfo, slot) => {
  selectedCourse.value = {
    ...courseInfo,
    course_number: slot.course_number,
    weekday: slot.weekday,
  };
  showDialog.value = true;
};

// 关闭对话框
const closeDialog = () => {
  showDialog.value = false;
  selectedCourse.value = null;
};

// 构建课程网格数据（处理合并单元格）
const courseGrid = computed(() => {
  const grid = {};

  // 初始化网格
  for (let day = 1; day <= 7; day++) {
    grid[day] = {};
    for (let courseNum = 1; courseNum <= 10; courseNum++) {
      grid[day][courseNum] = null;
    }
  }

  // 填充课程数据
  currentWeekData.value.forEach((dayCourse) => {
    const day = dayCourse.weekday;
    dayCourse.course.forEach((slot) => {
      if (slot.course_info) {
        const courseNum = slot.course_number;
        const courseInfo = slot.course_info;

        // 检查是否需要合并（相同 code 的连续课程）
        let mergeCount = 1;
        const pairGroups = [
          [1, 2],
          [3, 4],
          [5, 6],
          [7, 8],
          [9, 10],
        ];

        for (const group of pairGroups) {
          if (group.includes(courseNum)) {
            const otherNum = group.find((n) => n !== courseNum);
            const otherSlot = dayCourse.course.find(
              (s) => s.course_number === otherNum
            );

            if (otherSlot?.course_info?.code === courseInfo.code) {
              // 只在第一个节次显示合并的课程
              if (courseNum === group[0]) {
                mergeCount = 2;
                // 标记第二个节次为"已合并"
                grid[day][otherNum] = "merged";
              } else {
                // 第二个节次跳过（已被合并）
                return;
              }
            }
            break;
          }
        }

        grid[day][courseNum] = {
          ...courseInfo,
          course_number: courseNum,
          weekday: day,
          rowSpan: mergeCount,
        };
      }
    });
  });

  return grid;
});
</script>

<template>
  <div
    v-motion
    :initial="{ opacity: 0, y: 20 }"
    :enter="{
      opacity: 1,
      y: 0,
      transition: { duration: 600, delay: 200, ease: 'easeOut' },
    }"
    class="bg-white dark:bg-gray-900 rounded-2xl border-[0.5px] border-gray-200 dark:border-gray-800 overflow-hidden h-full flex flex-col"
  >
    <!-- 周选择器 -->
    <div class="border-b-[0.5px] border-gray-200 dark:border-gray-800 p-4 bg-gradient-to-r from-gray-50 to-white dark:from-gray-900 dark:to-gray-900">
      <div class="flex items-center gap-2 overflow-x-auto pb-2">
        <span class="text-sm text-gray-600 dark:text-gray-300 whitespace-nowrap mr-2 font-medium">选择周次:</span>
        <div class="flex gap-2">
          <button
            v-for="week in weeks"
            :key="week"
            @click="changeWeek(week)"
            :class="
              cn(
                'px-4 py-2 rounded-xl text-sm font-medium transition-colors duration-200',
                'whitespace-nowrap',
                currentWeek === week
                  ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900'
                  : 'bg-gray-100 text-gray-900 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-100 dark:hover:bg-gray-700'
              )
            "
          >
            第{{ week }}周
          </button>
        </div>
      </div>
    </div>

    <!-- 课表网格 -->
    <div class="p-4 flex-1 min-h-0">
      <div class="w-full sm:w-full sm:min-w-[740px]">
        <!-- 表头 -->
        <div class="grid grid-cols-8 gap-2 sm:gap-3 mb-3 timetableHeader">
          <div class="text-center text-sm font-semibold text-gray-600 dark:text-gray-300 py-3 bg-gradient-to-br from-gray-100 to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl">
            节次
          </div>
          <div
            v-for="day in 7"
            :key="day"
            class="text-center text-sm font-semibold text-gray-900 dark:text-gray-100 py-3 bg-gradient-to-br from-gray-100 to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl"
          >
            {{ weekdayNames[day] }}
          </div>
        </div>

        <!-- 课程表格 -->
        <div
          class="grid grid-cols-8 gap-2 sm:gap-3 h-full timetableGrid"
          style="height: 100%; grid-auto-rows: 1fr;"
        >
          <!-- 渲染所有格子 -->
          <template v-for="courseNum in 10" :key="courseNum">
            <!-- 节次列 -->
            <div
              class="flex flex-col items-center justify-center bg-gradient-to-br from-gray-100 to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl p-3"
              :style="{
                gridColumn: 1,
                gridRow: courseNum,
              }"
            >
              <div class="text-sm font-bold text-gray-900">
                {{ courseNum }}
              </div>
              <div class="text-[10px] text-gray-600 mt-1 text-center leading-tight">
                {{ courseTimeMap[courseNum] }}
              </div>
            </div>

            <!-- 每天的课程 -->
            <template v-for="day in 7" :key="`${courseNum}-${day}`">
              <!-- 有课程数据且不是被合并的格子 -->
              <div
                v-if="
                  courseGrid[day][courseNum] &&
                  courseGrid[day][courseNum] !== 'merged'
                "
                v-motion
                :initial="{ opacity: 0, scale: 0.9 }"
                :enter="{
                  opacity: 1,
                  scale: 1,
                  transition: { duration: 400, delay: courseNum * 30 + day * 20 },
                }"
                :class="
                  cn(
                    'p-3 rounded-xl text-white text-xs',
                    'transition-colors duration-200',
                    'cursor-pointer flex flex-col justify-center',
                    'backdrop-blur-sm'
                  )
                "
                :style="{
                  background: `linear-gradient(135deg, ${courseGrid[day][courseNum].color} 0%, ${courseGrid[day][courseNum].color}dd 100%)`,
                  gridColumn: day + 1,
                  gridRow:
                    courseGrid[day][courseNum].rowSpan === 2
                      ? `${courseNum} / span 2`
                      : courseNum,
                }"
                @click="
                  openCourseDialog(courseGrid[day][courseNum], {
                    course_number: courseNum,
                    weekday: day,
                  })
                "
              >
                <div class="font-semibold mb-2 line-clamp-2 leading-tight">
                  {{ courseGrid[day][courseNum].name }}
                </div>
                <div class="text-[10px] opacity-95 space-y-1">
                  <div
                    v-if="courseGrid[day][courseNum].classroom"
                    class="flex items-center gap-1.5"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="11"
                      height="11"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      class="flex-shrink-0"
                    >
                      <path
                        d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"
                      />
                      <circle cx="12" cy="10" r="3" />
                    </svg>
                    <span class="truncate">{{
                      courseGrid[day][courseNum].classroom
                    }}</span>
                  </div>
                  <div
                    v-if="courseGrid[day][courseNum].teacher?.length"
                    class="flex items-center gap-1.5"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="11"
                      height="11"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      class="flex-shrink-0"
                    >
                      <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
                      <circle cx="12" cy="7" r="4" />
                    </svg>
                    <span class="truncate">{{
                      courseGrid[day][courseNum].teacher.join(", ")
                    }}</span>
                  </div>
                </div>
              </div>
              <!-- 空格子（但不是被合并的格子） -->
              <div
                v-else-if="
                  !courseGrid[day][courseNum] ||
                  courseGrid[day][courseNum] === null
                "
                class="rounded-xl border-[0.5px] border-gray-200 dark:border-gray-800 bg-gradient-to-br from-gray-50/50 to-white dark:from-gray-800/50 dark:to-gray-900 hover:bg-gray-50/80 dark:hover:bg-gray-800/80 transition-colors duration-200"
                :style="{
                  gridColumn: day + 1,
                  gridRow: courseNum,
                }"
              ></div>
              <!-- 被合并的格子不渲染任何东西 -->
            </template>
          </template>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-if="!currentWeekData || currentWeekData.length === 0"
      v-motion
      :initial="{ opacity: 0, y: 20 }"
      :enter="{ opacity: 1, y: 0, transition: { duration: 500 } }"
      class="p-16 text-center"
    >
      <svg class="w-16 h-16 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
      </svg>
      <div class="text-lg font-medium text-gray-900 mb-2">本周暂无课程</div>
      <div class="text-sm text-gray-500">享受你的空闲时光吧！</div>
    </div>
  </div>

  <!-- 课程详情对话框 -->
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-300"
      leave-active-class="transition-opacity duration-200"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="showDialog"
        class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
        @click="closeDialog"
      >
        <Transition
          enter-active-class="transition-all duration-300"
          leave-active-class="transition-all duration-200"
          enter-from-class="opacity-0 scale-95"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="showDialog && selectedCourse"
            class="bg-white rounded-2xl border-[0.5px] border-gray-200 max-w-md w-full overflow-hidden"
            @click.stop
          >
            <!-- 头部 -->
            <div
              class="p-6 text-white relative overflow-hidden"
              :style="{
                background: `linear-gradient(135deg, ${selectedCourse.color} 0%, ${selectedCourse.color}dd 100%)`
              }"
            >
              <div class="absolute inset-0 bg-gradient-to-br from-white/10 to-transparent"></div>
              <div class="relative flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-xl font-bold mb-2 leading-tight">
                    {{ selectedCourse.name }}
                  </h3>
                  <div class="text-sm opacity-95 flex items-center gap-2">
                    <span>{{ weekdayNames[selectedCourse.weekday] }}</span>
                    <span>·</span>
                    <span>第{{ selectedCourse.course_number }}节</span>
                  </div>
                </div>
                <button
                  @click="closeDialog"
                  class="ml-4 p-2 hover:bg-white/20 rounded-xl transition-colors duration-200"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                  </svg>
                </button>
              </div>
            </div>

            <!-- 内容 -->
            <div class="p-6 space-y-4">
              <!-- 课程代码 -->
              <div class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gray-100 to-gray-50 flex items-center justify-center flex-shrink-0">
                  <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"/>
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-500 mb-1 font-medium">课程代码</div>
                  <div class="text-sm font-semibold text-gray-900 font-mono">
                    {{ selectedCourse.code }}
                  </div>
                </div>
              </div>

              <!-- 班级 -->
              <div v-if="selectedCourse.class" class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gray-100 to-gray-50 flex items-center justify-center flex-shrink-0">
                  <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-500 mb-1 font-medium">班级</div>
                  <div class="text-sm font-semibold text-gray-900">
                    {{ selectedCourse.class }}
                  </div>
                </div>
              </div>

              <!-- 教室 -->
              <div v-if="selectedCourse.classroom" class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gray-100 to-gray-50 flex items-center justify-center flex-shrink-0">
                  <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-500 mb-1 font-medium">教室</div>
                  <div class="text-sm font-semibold text-gray-900">
                    {{ selectedCourse.classroom }}
                  </div>
                </div>
              </div>

              <!-- 教师 -->
              <div v-if="selectedCourse.teacher?.length" class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gray-100 to-gray-50 flex items-center justify-center flex-shrink-0">
                  <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-500 mb-1 font-medium">教师</div>
                  <div class="text-sm font-semibold text-gray-900">
                    {{ selectedCourse.teacher.join(", ") }}
                  </div>
                </div>
              </div>

              <!-- 时间 -->
              <div class="flex items-start gap-3">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-gray-100 to-gray-50 flex items-center justify-center flex-shrink-0">
                  <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-500 mb-1 font-medium">上课时间</div>
                  <div class="text-sm font-semibold text-gray-900">
                    {{ courseTimeMap[selectedCourse.course_number] }}
                    <span v-if="selectedCourse.rowSpan > 1" class="text-xs text-gray-500 ml-2">
                      ({{ selectedCourse.rowSpan }}节连上)
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 底部 -->
            <div class="px-6 py-4 bg-gradient-to-r from-gray-50 to-white border-t-[0.5px] border-gray-200">
              <button
                @click="closeDialog"
                class="w-full py-3 px-4 bg-gradient-to-r from-gray-900 to-gray-700 text-white rounded-xl font-medium transition-colors duration-200"
              >
                关闭
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
/* 自定义滚动条（单色） */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db; /* gray-300 */
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #d1d5db; /* 保持单色，无过渡 */
}


/* PC 默认列宽（保持现状） */
.timetableHeader { grid-template-columns: minmax(56px,1fr) repeat(7, minmax(44px,1fr)); }
.timetableGrid  { grid-template-columns: minmax(56px,1fr) repeat(7, minmax(44px,1fr)); }

/* Mobile 优化：增大每列宽度，并启用横向滚动容器（已在上层加 overflow-x-auto） */
@media (max-width: 640px) {
  .timetableHeader { grid-template-columns: 48px repeat(7, 1fr); }
  .timetableGrid  { grid-template-columns: 48px repeat(7, 1fr); }
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
