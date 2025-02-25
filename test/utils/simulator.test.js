require('dotenv').config();
const simulator = require('../../utils/simulator');

(async function () {
  try {
    const { basicAuthValue, bearerAuthValue } = await simulator.startSimulator();
    console.log({ basicAuthValue, bearerAuthValue });
  } catch (error) {
    console.error('Error occurred:', error);
  }
})();
