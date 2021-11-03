FROM node:slim
ENV NODE_OPTIONS=--openssl-legacy-provider

WORKDIR /src
COPY . .
RUN yarn install
EXPOSE 8000

CMD [ "yarn", "start" ]
