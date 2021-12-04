import cv2
import os
import numpy as np
file_list = []

for root, dirs, files in os.walk("/home/tsundere-arch/NodeJsProjects/hackathon/PryC/backend/imgs/"):
   for file in files:
       if file.split(".")[1] != "py":
         dupa = os.path.join(root,file)
         print(dupa);
         print(root);
         image= cv2.imread(dupa)
         print('The image size of the original image is ', image.shape)
         scaled_down= cv2.pyrDown(image)
         cv2.imwrite(root+file, scaled_down)
