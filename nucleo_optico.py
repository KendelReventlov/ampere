import cv2
import socket

objeto_video = cv2.VideoCapture(0)
objeto_video.set(cv2.CAP_PROP_FRAME_WIDTH,720)
objeto_video.set(cv2.CAP_PROP_FRAME_HEIGHT,480)
exito = 1

while exito:
    conexion = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
    conexion.connect(("127.0.0.1",3000))
    exito,imagen = objeto_video.read()
    bytes = cv2.imencode(".png",imagen)[1].tobytes()
    conexion.send(bytes)
    conexion.close()
    