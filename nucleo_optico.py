import cv2
import numpy as np

objeto_video = cv2.VideoCapture(0)
objeto_video.set(cv2.CAP_PROP_FRAME_WIDTH,720)
objeto_video.set(cv2.CAP_PROP_FRAME_HEIGHT,480)
exito = 1

while exito:
    exito,imagen = objeto_video.read()
    print(np.array(imagen).shape)
    print()