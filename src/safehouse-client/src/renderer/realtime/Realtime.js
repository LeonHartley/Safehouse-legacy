import Auth from '../api/auth/Auth'

var connection = {
  socket: null,
  server: null
}

var connectionReady = (event) => {
  console.log('Safehouse-Realtime - Ready for messages')

  sendMessage({ message: 1, payload: { token: Auth.getAuthToken() } })
}

var handleMessage = (event) => {
  var data = JSON.parse(event.data)

  // Handle messages differently.
  console.log(data)
}

var sendMessage = (message) => {
  if (connection.socket.readyState === 1) {
    connection.socket.send(JSON.stringify(message))
  }
}

var disconnect = () => {
  connection.socket.close()
}

export default {
  connect (server) {
    connection.server = server
    connection.socket = new WebSocket('ws://' + server.host + ':' + server.port)

    connection.socket.onmessage = handleMessage
    connection.socket.onopen = connectionReady
  },

  send: sendMessage,

  disconnect: disconnect
}
