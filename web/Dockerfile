FROM node:16

WORKDIR /usr/src/trajectory-web
COPY . .

RUN npm install -g pnpm
RUN pnpm install
RUN pnpm run build

EXPOSE 5000
ENV HOST=0.0.0.0

CMD ["pnpm", "run", "serve"]
