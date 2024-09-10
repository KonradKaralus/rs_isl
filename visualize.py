from PIL import Image
import numpy as np
import cv2, os

def convert(f:float) -> list[int,int,int]:
    if f == 0:
        return (0,0,0)
    return (0,0,int(f))

def flatten(line:list[list[int,int,int]]) -> list[int]:
    out = []
    for l in line:
        out.append(l[0])
        out.append(l[1])
        out.append(l[2])

    return out

files = [int(f.replace("file","")) for f in os.listdir("output/raw")]
num_files = max(files) + 1

files = []
for i in range(num_files):
    print(f"at input file {i}")
    l = []
    with open(f"output/raw/file{i}","r") as f:
        items = f.readlines()
        l = [x.split(",") for x in items]
        for line in l:
            line.remove("\n")
    l = [[convert(float(x)) for x in line] for line in l]

    image = Image.fromarray(np.array(l).astype(np.uint8))
    image.save(f"output/img{i}.png")

images = ["output/"+img for img in os.listdir("./output") if img.endswith(".png")]

frame = cv2.imread(images[0])
height, width, layers = frame.shape

video = cv2.VideoWriter("export.avi", 0, 1, (width,height))

for (idx,image) in enumerate(images):
    print(f"at img {idx}")
    video.write(cv2.imread(image))
    
cv2.destroyAllWindows()
video.release()

for img in images:
    os.remove(img)