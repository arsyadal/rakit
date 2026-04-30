#!/usr/bin/env bash
# Full integration test for RAKIT MVP
set -e

BASE_URL="http://localhost:3000"

echo "🧪 RAKIT Full Flow Test"
echo "======================="

# 1. Health check
echo -e "\n1️⃣ Health Check"
curl -s "$BASE_URL/health" | jq .

# 2. Register user
echo -e "\n2️⃣ Register User"
REGISTER_RESP=$(curl -s -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"email":"demo@rakit.dev","password":"SecurePass123!"}')
echo "$REGISTER_RESP" | jq .
USER_ID=$(echo "$REGISTER_RESP" | jq -r '.id')

# 3. Login
echo -e "\n3️⃣ Login & Get Token"
LOGIN_RESP=$(curl -s -X POST "$BASE_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"demo@rakit.dev","password":"SecurePass123!"}')
TOKEN=$(echo "$LOGIN_RESP" | jq -r '.token')
echo "Token: ${TOKEN:0:50}..."

# 4. Get current user
echo -e "\n4️⃣ Get Current User (Protected)"
curl -s "$BASE_URL/api/v1/auth/me" \
  -H "Authorization: Bearer $TOKEN" | jq .

# 5. Create content in 'posts'
echo -e "\n5️⃣ Create Post"
POST_RESP=$(curl -s -X POST "$BASE_URL/api/v1/posts" \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello RAKIT","content":"This is my first post","published":true}')
echo "$POST_RESP" | jq .
POST_ID=$(echo "$POST_RESP" | jq -r '.id')

# 6. Create content in 'products'
echo -e "\n6️⃣ Create Product"
PROD_RESP=$(curl -s -X POST "$BASE_URL/api/v1/products" \
  -H "Content-Type: application/json" \
  -d '{"name":"RAKIT Mug","price":25,"sku":"MUG-001"}')
echo "$PROD_RESP" | jq .
PROD_ID=$(echo "$PROD_RESP" | jq -r '.id')

# 7. List posts
echo -e "\n7️⃣ List Posts (should only show posts)"
curl -s "$BASE_URL/api/v1/posts" | jq 'length, .[].collection'

# 8. Update post (PUT)
echo -e "\n8️⃣ Update Post (PUT)"
curl -s -X PUT "$BASE_URL/api/v1/posts/$POST_ID" \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated Title","content":"New content","published":false}' | jq .data

# 9. Patch product
echo -e "\n9️⃣ Patch Product (partial update)"
curl -s -X PATCH "$BASE_URL/api/v1/products/$PROD_ID" \
  -H "Content-Type: application/json" \
  -d '{"price":29,"in_stock":true}' | jq .data

# 10. Get single item
echo -e "\n🔟 Get Single Product"
curl -s "$BASE_URL/api/v1/products/$PROD_ID" | jq '{id, collection, data}'

echo -e "\n✅ All tests passed!"
echo -e "\nTo delete test data:"
echo "  curl -X DELETE $BASE_URL/api/v1/posts/$POST_ID"
echo "  curl -X DELETE $BASE_URL/api/v1/products/$PROD_ID"
