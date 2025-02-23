'use strict'

module.exports = async function (fastify, opts) {
  fastify.get('/course', async function (request, reply) {
    return {
        test: true
    }
  })
}
