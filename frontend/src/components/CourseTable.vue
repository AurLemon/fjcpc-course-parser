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
const weekdayNames = [
  "",
  "周一",
  "周二",
  "周三",
  "周四",
  "周五",
  "周六",
  "周日",
];

// 课程节次时间映射（使用后端返回的时间表）
const courseTimeMap = computed(() => {
  if (!props.scheduleData?.time_table) {
    // 默认冬季时间表
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
    class="bg-white rounded-2xl shadow-sm border-[0.5px] border-gray-200 overflow-hidden"
  >
    <!-- 周选择器 -->
    <div class="border-b-[0.5px] border-gray-200 p-4">
      <div class="flex items-center gap-2 overflow-x-auto pb-2">
        <span class="text-sm text-gray-600 whitespace-nowrap mr-2"
          >选择周次:</span
        >
        <div class="flex gap-2">
          <button
            v-for="week in weeks"
            :key="week"
            @click="changeWeek(week)"
            :class="
              cn(
                'px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
                'hover:scale-105 active:scale-95 whitespace-nowrap',
                currentWeek === week
                  ? 'bg-gray-900 text-white shadow-sm'
                  : 'bg-gray-100 text-gray-900 hover:bg-gray-200'
              )
            "
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
        <div
          class="grid grid-cols-8 gap-2"
          style="grid-auto-rows: minmax(60px, auto)"
        >
          <!-- 渲染所有格子 -->
          <template v-for="courseNum in 10" :key="courseNum">
            <!-- 节次列 -->
            <div
              class="flex flex-col items-center justify-center bg-gray-100 rounded-lg p-2"
              :style="{
                gridColumn: 1,
                gridRow: courseNum,
              }"
            >
              <div class="text-xs font-medium text-gray-900">
                {{ courseNum }}
              </div>
              <div class="text-[10px] text-gray-600 mt-0.5">
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
                  transition: { duration: 300, delay: 50 },
                }"
                :class="
                  cn(
                    'p-3 rounded-lg text-white text-xs',
                    'hover:shadow-lg hover:scale-[1.02] transition-all duration-200',
                    'cursor-pointer flex flex-col justify-center'
                  )
                "
                :style="{
                  backgroundColor: courseGrid[day][courseNum].color,
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
                <div class="font-medium mb-1.5 line-clamp-2">
                  {{ courseGrid[day][courseNum].name }}
                </div>
                <div class="text-[10px] opacity-90 space-y-0.5">
                  <div
                    v-if="courseGrid[day][courseNum].classroom"
                    class="flex items-center gap-1"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="10"
                      height="10"
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
                    class="flex items-center gap-1"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="10"
                      height="10"
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
                class="rounded-lg border-[0.5px] border-gray-200 bg-gray-50/30"
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
      class="p-12 text-center text-gray-600"
    >
      <div class="text-4xl mb-2">📅</div>
      <div class="text-sm">本周暂无课程</div>
    </div>
  </div>

  <!-- 课程详情对话框 -->
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-200"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="showDialog"
        class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
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
            class="bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden"
            @click.stop
          >
            <!-- 头部 -->
            <div
              class="p-6 text-white"
              :style="{ backgroundColor: selectedCourse.color }"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-xl font-bold mb-2">
                    {{ selectedCourse.name }}
                  </h3>
                  <div class="text-sm opacity-90">
                    {{ weekdayNames[selectedCourse.weekday] }} 第{{
                      selectedCourse.course_number
                    }}节
                  </div>
                </div>
                <button
                  @click="closeDialog"
                  class="ml-4 p-2 hover:bg-white/20 rounded-lg transition-colors"
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
            </div>

            <!-- 内容 -->
            <div class="p-6 space-y-4">
              <!-- 课程代码 -->
              <div class="flex items-start gap-3">
                <div
                  class="w-8 h-8 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                >
                  <svg
                    class="w-4 h-4 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"
                    />
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-600 mb-1">课程代码</div>
                  <div class="text-sm font-medium text-gray-900 font-mono">
                    {{ selectedCourse.code }}
                  </div>
                </div>
              </div>

              <!-- 班级 -->
              <div v-if="selectedCourse.class" class="flex items-start gap-3">
                <div
                  class="w-8 h-8 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                >
                  <svg
                    class="w-4 h-4 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                    />
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-600 mb-1">班级</div>
                  <div class="text-sm font-medium text-gray-900">
                    {{ selectedCourse.class }}
                  </div>
                </div>
              </div>

              <!-- 教室 -->
              <div
                v-if="selectedCourse.classroom"
                class="flex items-start gap-3"
              >
                <div
                  class="w-8 h-8 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                >
                  <svg
                    class="w-4 h-4 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
                    />
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
                    />
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-600 mb-1">教室</div>
                  <div class="text-sm font-medium text-gray-900">
                    {{ selectedCourse.classroom }}
                  </div>
                </div>
              </div>

              <!-- 教师 -->
              <div
                v-if="selectedCourse.teacher?.length"
                class="flex items-start gap-3"
              >
                <div
                  class="w-8 h-8 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                >
                  <svg
                    class="w-4 h-4 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                    />
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-600 mb-1">教师</div>
                  <div class="text-sm font-medium text-gray-900">
                    {{ selectedCourse.teacher.join(", ") }}
                  </div>
                </div>
              </div>

              <!-- 时间 -->
              <div class="flex items-start gap-3">
                <div
                  class="w-8 h-8 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                >
                  <svg
                    class="w-4 h-4 text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                </div>
                <div class="flex-1">
                  <div class="text-xs text-gray-600 mb-1">上课时间</div>
                  <div class="text-sm font-medium text-gray-900">
                    {{ courseTimeMap[selectedCourse.course_number] }}
                    <span
                      v-if="selectedCourse.rowSpan > 1"
                      class="text-xs text-gray-600 ml-2"
                    >
                      ({{ selectedCourse.rowSpan }}节连上)
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 底部 -->
            <div class="px-6 py-4 bg-gray-50 border-t border-gray-200">
              <button
                @click="closeDialog"
                class="w-full py-2.5 px-4 bg-gray-900 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
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
