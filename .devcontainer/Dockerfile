FROM ros:noetic-robot

RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 0 && \
  adduser --disabled-password rosuser && \
  echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

USER rosuser
WORKDIR /home/rosuser

RUN . /opt/ros/noetic/setup.sh

EXPOSE 11311
EXPOSE 9090
EXPOSE 2112