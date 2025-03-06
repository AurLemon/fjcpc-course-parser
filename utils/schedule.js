// utils/schedule.js
'use strict';

/**
 * 日程表对象
 * 
 * 根据学校作息时间表删减后手动转储为对象文件，考虑到项目只服务于我们船政所以就写死成导出模块到项目里方便直接用了
 * 
 * @author AurLemon
 */
const schedule = {
  basicInfo: {
    // 每天课程数
    totalLessons: 10,

    // 课件时长，单位分钟
    lessonDuration: 45
  }
};

module.exports = schedule;
