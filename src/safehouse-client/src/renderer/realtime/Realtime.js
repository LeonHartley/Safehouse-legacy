import Auth from '../api/auth/Auth'
import Message from './Message'
import Store from '../store'

var connection = {
  socket: null,
  server: null
}

var connectionReady = (event) => {
  console.log('Safehouse-Realtime - Ready for messages')

  sendMessage(new Message(1, Auth.getAuthToken()))

  // TODO: Remove this once we have status updates done ;-)
  setInterval(() => {
    sendMessage(new Message(2))
  }, 3000)
}

var handleMessage = (event) => {
  getBuffer(event.data, (buffer) => {
    var msg = Message.decode(buffer)

    if (msg.type === 2) {
      // contact status update
      Store.commit('updateContacts', JSON.parse(msg.payload))
    }

    console.log('msg type: ' + msg.type + ', payload: ' + msg.payload)
  })
}

var sendMessage = (message) => {
  if (connection.socket.readyState === 1) {
    // todo: expandable buffer..
    connection.socket.send(message.encode(new ArrayBuffer(1024)))
  }
}

var disconnect = () => {
  connection.socket.close()
}

function getBuffer (blob, consumer) {
  var fileReader = new FileReader()

  fileReader.onload = function () {
    consumer(this.result)
  }

  fileReader.readAsArrayBuffer(blob)
}

export default {
  connect (server) {
    connection.server = server
    connection.socket = new WebSocket('ws://' + server.host + ':' + server.port)

    connection.socket.onmessage = handleMessage
    connection.socket.onopen = (e) => {
      connectionReady(e)
      server.ready()
    }
  },

  send: sendMessage,

  disconnect: disconnect,
  Message: Message
}
