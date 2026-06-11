KEYCLOAK_URL="https://websso.dev.openanalytics.eu/auth"
REALM="openanalytics"
CLIENT_ID="nix-cache"

# Construct the token endpoint URL
TOKEN_ENDPOINT="${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/token"

echo "Requesting token from: $TOKEN_ENDPOINT..."

# --- Fetch the Token ---
# We use -s for silent mode, then post the required x-www-form-urlencoded payload
RESPONSE=$(curl -s -X POST "$TOKEN_ENDPOINT" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=client_credentials" \
    -d "client_id=${CLIENT_ID}" \
    -d "client_secret=${CLIENT_SECRET}")

# --- Extract the Access Token ---
# jq -r extracts the raw string value of the access_token field
ACCESS_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token // empty')

# --- Validation ---
if [ -n "$ACCESS_TOKEN" ]; then
    echo "✅ Successfully retrieved access token!"
    export ACCESS_TOKEN="$ACCESS_TOKEN"
    echo "$ACCESS_TOKEN"
else
    echo "❌ Failed to retrieve access token. Keycloak response:"
    echo "$RESPONSE" | jq .
    exit 1
fi