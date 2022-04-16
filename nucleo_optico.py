import cv2
import numpy as np

objeto_video = cv2.VideoCapture(0)
exito = 1

while exito:
    exito,imagen = objeto_video.read()
    print(np.array(imagen).shape)
    print()