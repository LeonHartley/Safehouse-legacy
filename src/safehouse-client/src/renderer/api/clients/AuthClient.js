import ApiClient from './ApiClient'

export default class AuthClient extends ApiClient {
  authorise (username, password) {
    return this.post('/authorise', {
      username: username,
      password: password
    })
  }
}
