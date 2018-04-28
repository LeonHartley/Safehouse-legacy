// Clients
import AuthClient from './clients/AuthClient'
import UserClient from './clients/UserClient'

const clientConfig = {
  baseUrl: 'http://localhost:1337'
}

export default {
  auth: new AuthClient(clientConfig),
  user: new UserClient(clientConfig)
}
