import cv2

objeto_video = cv2.VideoCapture(0)
exito = 1

while exito:
    exito,imagen = objeto_video.read()
    print(imagen)
    print()