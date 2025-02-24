// services/courseService.js

const dayjs = require('dayjs');
const axios = require('axios');

const config = require('../utils/config');
const authService = require('./authService');

/**
 * 获取系统内有记录的学年数据（学期起始日和结束日、周数）
 * @returns {Promise<Array>}
 */
const getAllSemester = async () => {

}

/**
 * 获取指定学期所有周的起始日
 * @param {string} semester 
 */
const getWeekStartsBySemester = async (semester) => {

}

/**
 * 获取周课程
 * @param {string} semester 
 * @param {string} week 
 */
const getWeekCourse = async (semester, week) => {

}

/**
 * 解析课程信息（把课程信息字符串解析成可读的对象）
 * @param {string} string
 */
const parseCourseString = async (string) => {

}

module.exports = { getAllSemester, getWeekStartsBySemester, getWeekCourse };
