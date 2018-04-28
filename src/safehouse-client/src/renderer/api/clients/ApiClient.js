import Auth from '../auth/Auth'
import Axios from 'axios'

// Axios.interceptors.response.use(function (response) {
//   if (response.data.token) {
//     if (response.data.token.invalidated) {
//       Auth.clearAuthentication()
//     } else {
//       Auth.setAuthentication(response.data.token.token)
//     }
//   }

//   return response
// }, function (error) {
//   return Promise.reject(error)
// })

export default class ApiClient {
  constructor (config) {
    this.config = config
  }

  get (path) {
    return Axios.get(this.config.baseUrl + path, this.axiosConfig())
  }

  post (path, payload) {
    return Axios.post(this.config.baseUrl + path, payload, this.axiosConfig())
  }

  getConfig () {
    return this.config
  }

  axiosConfig () {
    var headers = {}

    if (Auth.isAuthenticated()) {
      headers['Authorization'] = 'Bearer ' + Auth.getAuthToken()
    }

    console.log(headers)

    return {
      headers: headers
    }
  }
}
