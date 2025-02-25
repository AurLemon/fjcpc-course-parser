require('dotenv').config();
const authService = require('../../services/authService');

(async function () {
  const TEST_UCODE = process.env.TEST_STUDENT_UCODE;
  
  try {
    const userInfo = await authService.getUserInfo(TEST_UCODE);
    console.log("userInfo: ", userInfo);

    const serverBasicAuth = await authService.getServerBasicAuth(TEST_UCODE);
    console.log("serverBasicAuth: " + serverBasicAuth);
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
