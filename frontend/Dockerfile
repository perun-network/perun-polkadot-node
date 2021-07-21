FROM node:slim

WORKDIR /src
COPY . .
RUN yarn install
EXPOSE 8000

CMD [ "yarn", "start" ]
