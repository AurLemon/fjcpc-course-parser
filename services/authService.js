// services/authService.js
'use strict';

const config = require('../utils/config');
const api = require('../utils/api');
const simulator = require('../utils/simulator');

const axiosIPv4 = api.axiosIPv4();

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
 * @param {string} rawUcode
 * @returns {Promise<string>}
 */
const getServerBasicAuth = async (rawUcode) => {
  const { basicAuthValue } = await simulator.startSimulator(rawUcode);
  return basicAuthValue;
};

/**
 * 传入 UCode 以获得用户信息
 * @param {string} rawUcode 不带前缀的用户标识码
 * @returns {Promise<{accessToken: string, refreshToken: string, studentId: string, studentPhone: string, studentRealname: string}>}
 */
const getUserInfo = async (rawUcode) => {
  const requestUrl = config.collegeAppBaseUrl + '/gateway/auth/oauth/token';
  const ucode = `HUA_TENG-${rawUcode}`;

  const requestParams = {
    ucode,
    state: 1,
    grant_type: 'ucode',
    scope: 'server',
  };
  
  try {
    const response = await axiosIPv4.get(requestUrl, {
      headers: {
        Authorization: getBasicAuth()
      },
      params: requestParams
    });

    const accessToken = response.data.access_token;
    const refreshToken = response.data.refresh_token;

    const studentId = response.data.user_info.username;
    const studentPhone = response.data.user_info.phone;
    const studentRealname = response.data.user_info.nickName;

    return { accessToken, refreshToken, studentId, studentPhone, studentRealname };
  } catch (error) {
    return await api.authRequestRetry(error, async () => {      
      const response = await axiosIPv4.get(requestUrl, {
        headers: {
          Authorization: await getServerBasicAuth()
        },
        params: requestParams
      });

      const accessToken = response.data.access_token;
      const refreshToken = response.data.refresh_token;

      const studentId = response.data.user_info.username;
      const studentPhone = response.data.user_info.phone;
      const studentRealname = response.data.user_info.nickName;

      return { accessToken, refreshToken, studentId, studentPhone, studentRealname };
    });
  }
};

module.exports = { getBasicAuth, getServerBasicAuth, getUserInfo };