# Cortex Protocol — Frontend

Next.js marketplace UI for Intelligence Rail.

## Stack

- **Next.js 16** (App Router)
- **TypeScript**
- **Tailwind CSS**

## Getting Started

```bash
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000).

## Structure

```
src/app/
├── layout.tsx      # Root layout with metadata
├── globals.css     # Global styles
├── page.tsx        # Landing page
└── marketplace/    # Asset browse & discovery (coming soon)
```

## Environment

No environment variables required for the frontend in development. For production, set:

```
NEXT_PUBLIC_API_URL=https://your-backend-url
NEXT_PUBLIC_STELLAR_NETWORK=testnet
```
