// test/services/scheduleService.test.js
'use strict';

require('dotenv').config();

const scheduleService = require('../../services/scheduleService');
const authService = require('../../services/authService');

const TEST_UCODE = process.env.TEST_STUDENT_UCODE;

(async () => {
  const userInfo = await authService.getUserInfo(TEST_UCODE);

  try {
    const schoolYear = await scheduleService.getSchoolYear(userInfo.accessToken);
    console.log("School year:", schoolYear);

    const currentSemester = schoolYear.find(s => s.is_current_semester);
    if (!currentSemester) return;
    const semester = await scheduleService.getSemester(userInfo.accessToken, currentSemester.school_year, currentSemester.semester);
    console.log("Semester:", semester);

    const weekCourse = await scheduleService.getWeekCourse(userInfo.accessToken, userInfo.studentId, currentSemester.start_time);
    console.log("Week course:", JSON.stringify(weekCourse, null, 2));
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
