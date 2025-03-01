require('dotenv').config();

const courseService = require('../../services/courseService');
const authService = require('../../services/authService');

const TEST_UCODE = process.env.TEST_STUDENT_UCODE;

(async () => {
  const userInfo = await authService.getUserInfo(TEST_UCODE);
  
  try {
    const schoolYear = await courseService.getSchoolYear(userInfo.accessToken);
    console.log(schoolYear);

    const semester = await courseService.getSemester(userInfo.accessToken, '2024-2025', '2');
    console.log(semester);
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
