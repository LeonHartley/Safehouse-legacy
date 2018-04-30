export default class Message {
  constructor (type, payload = '') {
    this.type = type
    this.payload = payload
  }

  encode (buffer) {
    var index = 0
    var view = new DataView(buffer)

    // soon this will be encrypted. ;-D
    view.setUint16(index, this.type)
    view.setUint16(index += 2, this.payload.length)

    index += 1

    var payloadBuf = new TextEncoder('utf-8').encode(this.payload)

    for (var i = 0; i < payloadBuf.length + 1; i++) {
      view.setUint8(index += 1, payloadBuf[i])
    }

    view.setUint8(index += 1, 0)

    return buffer.slice(0, index)
  }

  static decode (buffer) {
    var view = new DataView(buffer)
    var index = 0
    var type = view.getUint16(index)
    var payloadLength = view.getUint16(index += 2)

    index += 1

    var payloadArray = new Uint8Array(payloadLength)

    for (var i = 0; i < payloadLength; i++) {
      payloadArray[i] = view.getUint8(index += 1)
    }

    return new Message(type, new TextDecoder('utf-8').decode(payloadArray))
  }
}
