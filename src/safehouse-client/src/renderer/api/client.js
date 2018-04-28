// Clients
import AuthClient from './clients/AuthClient'

const clientConfig = {
  baseUrl: 'http://localhost:1337'
}

export default {
  auth: new AuthClient(clientConfig)
}
