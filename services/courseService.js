// services/courseService.js
'use strict';

const scheduleService = require('./scheduleService');
const api = require('../utils/api');
const log = require('../utils/log');

const axiosIPv4 = api.axiosIPv4();

/**
 * 批量获取所有课程
 * @param {string} userToken 用户令牌
 * @param {string} studentId 学号
 * @param {Array<{week: number, start_time: string}>} semester 学期信息
 * @returns {Promise<Map<number, Array<{course_name: string, course_type: string, course_time: string, course_place: string, course_teacher: string,course_week: string, course_weekday: string, course_week}}
 */
const getAllCourses = async (userToken, studentId, semester) => {
  if (!Array.isArray(semester)) {
    throw new Error("Semester info must be an array.");
  }

  if (!studentId || !userToken) {
    throw new Error("Student ID or User token must be provided.");
  }

  const coursesMap = new Map();
  const promises = semester.map(async (week) => {
    try {
      log.info(`Student ${studentId} (${userToken}) are requesting week course. (${week.week} / ${semester.length})`);
      const weekCourseData = await scheduleService.getWeekCourse(userToken, studentId, week.start_time);
      coursesMap.set(week.week, weekCourseData);
      log.info(`Student ${studentId} (${userToken}) requested week ${week.week} course successfully.`);
    } catch (error) {
      log.error(`Student ${studentId} (${userToken}) failed to request week course.`)
    }
  });
  
  await Promise.all(promises);

  return coursesMap;
};

module.exports = { getAllCourses };
