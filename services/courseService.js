// services/courseService.js
'use strict';

const scheduleService = require('./scheduleService');
const api = require('../utils/api');

const axiosIPv4 = api.axiosIPv4();
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
      console.log("Requesting week: " + week.week + ". Total: " + semester.length);
      const weekCourseData = await scheduleService.getWeekCourse(userToken, studentId, week.start_time);
      coursesMap.set(week.week, weekCourseData);
      console.log("Request finished for week:", week.week);
    } catch (error) {
      console.error(`Failed to fetch data for week ${week.week}:`, error);
    }
  });
  
  await Promise.all(promises);
  
  return coursesMap;
};

module.exports = { getAllCourses };