// services/authService.js

const axios = require('axios');

const config = require('../utils/config');
const simulator = require('../utils/simulator');

/**
 * 获取 Basic 验证字符串（根据校内服务器数据推测，存在不确定性）
 * @returns {string}
 */
const getBasicAuth = () => {
  const username = 'cat';
  const password = 'cat';
  return 'Basic ' + Buffer.from(`${username}:${password}`).toString('base64');
};

/**
 * 获取 Basic 验证字符串（模拟浏览器环境，直接模拟学生访问课表以获取现实数据，非必要不用）
 * @returns {Promise<string>}
 */
const getServerBasicAuth = async () => {
  const { basicAuthValue } = await simulator.startSimulator();
  return basicAuthValue;
};

/**
 * 传入 UCode 以获得用户信息
 * @param {string} rawUcode
 * @returns {Promise<{accessToken: string, refreshToken: string, studentId: string, studentPhone: string, studentRealname: string}>}
 */
const getUserInfo = async (rawUcode) => {
  const requestUrl = config.collegeAppBaseUrl + '/gateway/auth/oauth/token';
  const ucode = `HUA_TENG-${rawUcode}`;

  const requestBody = {
    ucode,
    state: 1,
    grant_type: ucode,
    scope: 'server',
  };

  try {
    const response = await axios.post(requestUrl, requestBody, {
      headers: {
        Authorization: getBasicAuth(),
      },
    });

    const accessToken = response.data.access_token;
    const refreshToken = response.data.refresh_token;

    const studentId = response.data.user_info.username;
    const studentPhone = response.data.user_info.phone;
    const studentRealname = response.data.user_info.nickname;

    return { accessToken, refreshToken, studentId, studentPhone, studentRealname };
  } catch (error) {
    if (error.response) {
      // 401 Unauthorized 尝试一次重试
      if (error.response.status === 401) {
        try {
          // 用浏览器环境模拟器尝试获取 Authorization
          const response = await axios.post(requestUrl, requestBody, {
            headers: {
              Authorization: getServerBasicAuth(),
            },
          });

          const accessToken = response.data.access_token;
          const refreshToken = response.data.refresh_token;

          const studentId = response.data.user_info.username;
          const studentPhone = response.data.user_info.phone;
          const studentRealname = response.data.user_info.nickname;

          return { accessToken, refreshToken, studentId, studentPhone, studentRealname };
        } catch (retryError) {
          // 还不行就算了
          throw new Error(`Error while fetching user token on retry: ${retryError.response.status}. Message: ${retryError.response.data.message}.`);
        }
      }
      
      throw new Error(`Error while fetching user token on retry: ${retryError.response.status}. Message: ${retryError.response.data.message}.`);
    } else if (error.request) {
      throw new Error('Error while fetching user token: ' + error.request);
    } else {
      throw new Error('Error while fetching user token: ' + error.message);
    }
  }
};

module.exports = { getBasicAuth, getServerBasicAuth, getUserInfo };