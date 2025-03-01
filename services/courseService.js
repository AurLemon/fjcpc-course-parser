// services/courseService.js

const dayjs = require('dayjs');
const axios = require('axios');

const config = require('../utils/config');
const api = require('../utils/api');

/**
 * 获取系统内有记录的学年数据（学期起始日和结束日、周数）
 * @param {string} userToken Token
 * @returns {Promise<Array>} 学年
 */
const getSchoolYear = async (userToken) => {
  const schoolYearUrl = `${config.collegeAppBaseUrl}/gateway/xgwork/appCourseTable/getXn`;

  try {
    const response = await axios.get(schoolYearUrl, {
      headers: {
        Authorization: `Bearer ${userToken}`
      }
    });

    const schoolYearData = response.data.data;

    const formattedData = schoolYearData.map(item => ({
      school_year: item.xn,
      semester: Number(item.xq),
      is_current_semester: Boolean(Number(item.dqxqbj)),
      start_time: item.qsrq,
      end_time: item.jsrq
    }));

    return formattedData;
  } catch (error) {
    return api.requestRetry(error);
  }
};

/**
 * 获取指定学期所有周的起始日
 * @param {string} userToken Token
 * @param {string} schoolYear 学年
 * @param {string} semester 学期
 * @returns {Promise<Array>} 学期周数据
 */
const getSemester = async (userToken, schoolYear, semester) => {
  const semesterUrl = `${config.collegeAppBaseUrl}/gateway/xgwork/appCourseTable/getSemesterbyXn`;
  const requestParams = {
    xn: schoolYear,
    xq: semester
  };

  try {
    const response = await axios.get(semesterUrl, {
      headers: {
        Authorization: `Bearer ${userToken}`
      },
      params: requestParams
    });

    const semesterData = response.data.data;

    const formattedData = semesterData.map(item => ({
      week: Number(item[0]),
      start_time: item[1],
      end_time: item[2]
    }));

    return formattedData;
  } catch (error) {
    return api.requestRetry(error);
  }
};

/**
 * 获取周课程
 * @param {string} userToken Token
 * @param {string} studentId 学号
 * @param {string} startTime 起始日
 * @returns {Promise<Array>} 周课程
 */
const getWeekCourse = async (userToken, studentId, startTime) => {
  const semesterUrl = `${config.collegeAppBaseUrl}/gateway/xgwork/appCourseTable/getListByNoWeek2`;
  const requestParams = {
    no: studentId,
    week: startTime
  };

  try {
    const response = await axios.get(semesterUrl, {
      headers: {
        Authorization: `Bearer ${userToken}`
      },
      params: requestParams
    });

    const weekCourseData = response.data.data;

    const formattedData = weekCourseData.map(item => ({
      
    }));

    return formattedData;
  } catch (error) {
    return api.requestRetry(error);
  }
};

/**
 * 解析课程信息（把课程信息字符串解析成可读的对象）
 * @param {string} string 课程信息字符串
 * @returns {Promise<Object>} 解析后的课程信息
 */
const parseCourseString = async (string) => {

};

module.exports = { getSchoolYear, getSemester, getWeekCourse };
