'use strict';

const responseSchema = require('../utils/schema');
const { successResponse } = require('../utils/response');

module.exports = async function (fastify, opts) {
  fastify.get('/course', {
    schema: {
      response: {
        200: responseSchema
      }
    }
  },
  async function (request, reply) {
    const data = {
      ucode: process.env.TEST_STUDENT_UCODE
    }
    return successResponse(200, data, 'Successfully get course info.')
  });
}
