from PIL import Image
import numpy as np

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

files = []
for i in range(50):
    print(f"at input file {i}")
    l = []
    with open(f"output/file{i}","r") as f:
        items = f.readlines()
        l = [x.split(",") for x in items]
        for line in l:
            line.remove("\n")
        # files.append(l)
    l = [[convert(float(x)) for x in line] for line in l]

    image = Image.fromarray(np.array(l).astype(np.uint8))
    # print(np.array(img))
    image.save(f"img{i}.png")
    



# # imgs = []

# # print(files)



# # files = [[convert(float(x)) for x in line] for img in files for line in img]
# files = [[[convert(float(x)) for x in line] for line in img] for img in files]

# # print(files)


# # files = [[flatten(line) for line in img] for img in files]

# def flatten2(line:list[list[int]]) -> list[int]:
#     out = []
#     for l in line:
#         for l2 in l:
#             out.append(l2)

#     return out

# images = []

# # print(files[0])
# for (i,img) in enumerate(files):
#     print(f"at img {i}")
#     # image = Image.fromarray((img * 1).astype(np.uint8)).convert('RGB')
#     image = Image.fromarray(np.array(img).astype(np.uint8))
#     # print(np.array(img))
#     image.save(f"img{i}.png")
#     # images.append(image)



import cv2, os

images = [img for img in os.listdir(".") if img.endswith(".png")]


frame = cv2.imread(images[0])
height, width, layers = frame.shape

video = cv2.VideoWriter("ani.avi", 0, 1, (width,height))

for (idx,image) in enumerate(images):
    print(f"at img {idx}")
    video.write(cv2.imread(image))
    
cv2.destroyAllWindows()
video.release()
