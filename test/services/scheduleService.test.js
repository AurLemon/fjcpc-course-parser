require('dotenv').config();

const scheduleService = require('../../services/scheduleService');
const authService = require('../../services/authService');

const TEST_UCODE = process.env.TEST_STUDENT_UCODE;

(async () => {
  const userInfo = await authService.getUserInfo(TEST_UCODE);

  try {
    const schoolYear = await scheduleService.getSchoolYear(userInfo.accessToken);
    console.log("School year:", schoolYear);

    const semester = await scheduleService.getSemester(userInfo.accessToken, '2024-2025', '2');
    console.log("Semester:", semester);

    const weekCourse = await scheduleService.getWeekCourse(userInfo.accessToken, userInfo.studentId, '2025-02-24');
    console.log("Week course:", JSON.stringify(weekCourse, null, 2));
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
