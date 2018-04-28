import ApiClient from './ApiClient'

export default class UserClient extends ApiClient {
  contacts () {
    return this.get('/contacts')
  }
}
