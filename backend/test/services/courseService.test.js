// test/services/courseService.test.js
'use strict';

require('dotenv').config();

const fs = require('fs');
const path = require('path');

const courseService = require('../../services/courseService');
const scheduleService = require('../../services/scheduleService');
const authService = require('../../services/authService');

const TEST_UCODE = process.env.TEST_STUDENT_UCODE;

(async () => {
  const userInfo = await authService.getUserInfo(TEST_UCODE);

  try {
    const schoolYear = await scheduleService.getSchoolYear(userInfo.accessToken);
    const currentSemester = schoolYear.findLast(s => s.is_current_semester);
    if (!currentSemester) return;
    const semester = await scheduleService.getSemester(userInfo.accessToken, currentSemester.school_year, currentSemester.semester);
    
    const filePath = path.join(__dirname, '../data/all_courses.json');
    if (!fs.existsSync(path.dirname(filePath))) fs.mkdirSync(path.dirname(filePath), { recursive: true });
    const allCourses = await courseService.getAllCourses(userInfo.accessToken, userInfo.studentId, semester);
    const sortedCourses = JSON.stringify(Object.fromEntries([...allCourses.entries()].sort(([a], [b]) => a - b)), null, 2);
    console.log("All courses:", sortedCourses);
    fs.writeFileSync(filePath, sortedCourses, 'utf8');
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
