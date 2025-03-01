// utils/utils.js

/**
 * 认证请求重试（懒得写一堆）
 * @param {AxiosError} error AxiosError 的错误对象
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
 * @param {AxiosError} error AxiosError 的错误对象
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

module.exports = { requestRetry, authRequestRetry };