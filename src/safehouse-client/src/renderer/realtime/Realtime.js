var connection = {
  socket: null,
  server: null
}

var connectionReady = (event) => {
  console.log('Safehouse-Realtime - Ready for messages')
}

var handleMessage = (event) => {
  var data = JSON.parse(event.data)

  // Handle messages differently.
  console.log(data)
}

export default {
  connect (server) {
    connection.server = server
    connection.socket = new WebSocket('ws://' + server.host + ':' + server.port)

    connection.socket.onmessage = handleMessage
    connection.socket.onopen = connectionReady
  },

  send (message) {
    if (connection.socket.readyState === 1) {
      connection.socket.send(JSON.stringify(message))
    }
  }
}
