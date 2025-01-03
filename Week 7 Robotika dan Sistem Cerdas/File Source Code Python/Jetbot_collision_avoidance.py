# Copyright 1996-2021 Cyberbotics Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""jetbot_collision_avoidance controller."""

# The code is taken from the Jupyter notebook at
# https://github.com/NVIDIA-AI-IOT/jetbot/blob/master/notebooks/collision_avoidance/live_demo_resnet18.ipynb

import torch
import torchvision.transforms as transforms
import torch.nn.functional as F
import torchvision
import PIL.Image
import os.path

from jetbot_python_control import JetBot

mean = torch.Tensor([0.485, 0.456, 0.406])
std = torch.Tensor([0.229, 0.224, 0.225])

normalize = torchvision.transforms.Normalize(mean, std)

model_path = 'best_model_resnet18.pth'


def preprocessCameraImage(camera):
    global device, normalize
    data = camera.getImage()
    image = PIL.Image.frombytes('RGBA', (camera.getWidth(), camera.getHeight()), data, 'raw', 'BGRA').convert('RGB')
    image = transforms.functional.to_tensor(image).to(device)
    image.sub_(mean[:, None, None]).div_(std[:, None, None])
    return image[None, ...]


if not os.path.isfile(model_path):
    print('Trained model "' + model_path + '" not found, please use the "jetbot_collect_data" controller to generate it.')
    exit()

# Create the Robot instance.
robot = JetBot()

# Set the controller time step
timestep = 5 * int(robot.getBasicTimeStep())

robot.camera.enable(timestep)

robot.step(10 * timestep)

print('Load the trained model..')
from torchvision.models import ResNet18_Weights
model = torchvision.models.resnet18(weights=ResNet18_Weights.DEFAULT)
model.fc = torch.nn.Linear(512, 2)
model.load_state_dict(torch.load(model_path, weights_only=True))
device = torch.device('cpu')
model = model.to(device)
model = model.eval()


# Main loop
print('Start collision avoidance control..')
direction = ''
while robot.step(timestep) != -1:
    camera_value = preprocessCameraImage(robot.camera)
    y = model(camera_value)
    # we apply the `softmax` function to normalize the output vector so it sums to 1 (which makes it a probability distribution)
    y = F.softmax(y, dim=1)
    prob_blocked = float(y.flatten()[0])
    if prob_blocked < 0.5:
        # Free path detected: move forward.
        if direction != 'forward':
            robot.forward(0.5)
            direction = 'forward'
    elif direction != 'left':
        # Edge detected: robot should turn!
        direction = 'left'
        robot.left(0.4)
    pass
