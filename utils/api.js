// utils/api.js
'use strict';

const axios = require('axios');
const http = require('http');
const https = require('https');

/**
 * 强制 IPv4 的 Axios 请求（为了解决船政那个神必 DNS 服务器加了 AAAA 却没法解析的问题）
 * @returns {import('axios').AxiosInstance}
 */
const axiosIPv4 = () => axios.create({
  httpAgent: new http.Agent({ family: 4 }),
  httpsAgent: new https.Agent({ family: 4 }),
});

/**
 * 认证请求重试（懒得写一堆）
 * @param {import('axios').AxiosError} error AxiosError 的错误对象
 */
const requestRetry = (error) => {
  console.error("Error while fetching info:", error);

  if (error.response) {    
    throw new Error(`Error while fetching info: ${error.response.status}. Message: ${error.response.data.message}.`);
  } else if (error.request) {
    throw new Error('Error while fetching info: ' + error.request);
  } else {
    throw new Error('Error while fetching info: ' + error.message);
  }
};

/**
 * 401 错误后认证请求重试（懒得写一堆）
 * @param {import('axios').AxiosError} error AxiosError 的错误对象
 * @param {Function} unauthRetry 验证错误后重试的回调函数
 * @returns {Promise<any>}
 */
const authRequestRetry = async (error, unauthRetry) => {
  console.error("Error while fetching info:", error);

  if (error.response) {
    // 401 Unauthorized 尝试一次重试
    if (error.response.status === 401) {
      try {
        return await unauthRetry();
      } catch (retryError) {
        throw new Error(`Error while fetching on retry: ${retryError.response.status}. Message: ${retryError.response.data.message}.`);
      }
    }
    
    throw new Error(`Error while fetching info: ${error.response.status}. Message: ${error.response.data.message}.`);
  } else if (error.request) {
    throw new Error('Error while fetching info: ' + error.request);
  } else {
    throw new Error('Error while fetching info: ' + error.message);
  }
};

module.exports = { axiosIPv4, requestRetry, authRequestRetry };