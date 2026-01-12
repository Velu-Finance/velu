FROM node:20-alpine AS builder

WORKDIR /app

# Yarn install (создаст yarn.lock если нет)
COPY package.json yarn.lock* ./
RUN corepack enable && yarn install --frozen-lockfile

COPY . .
RUN yarn build

FROM node:20-alpine
WORKDIR /app

# Corepack для yarn
RUN corepack enable

COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000
CMD ["yarn", "start"]
