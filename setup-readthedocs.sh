#!/bin/bash
# ReadTheDocs Setup Script for TaskGuard
# Usage: READTHEDOCS_API_KEY=your_key_here ./setup-readthedocs.sh

set -e

if [ -z "$READTHEDOCS_API_KEY" ]; then
    echo "Error: READTHEDOCS_API_KEY environment variable not set"
    echo "Usage: READTHEDOCS_API_KEY=your_key_here ./setup-readthedocs.sh"
    exit 1
fi

echo "🚀 Setting up TaskGuard on ReadTheDocs..."
echo ""

# Step 1: Check if project already exists
echo "📋 Checking if project exists..."
PROJECT_CHECK=$(curl -s -X GET \
  "https://readthedocs.org/api/v3/projects/" \
  -H "Authorization: Token $READTHEDOCS_API_KEY" \
  -H "Content-Type: application/json")

if echo "$PROJECT_CHECK" | grep -q "taskguard"; then
    echo "✅ Project 'taskguard' already exists"
    PROJECT_SLUG="taskguard"
else
    echo "📦 Creating new project..."

    # Step 2: Import the project
    CREATE_RESPONSE=$(curl -s -X POST \
      "https://readthedocs.org/api/v3/projects/" \
      -H "Authorization: Token $READTHEDOCS_API_KEY" \
      -H "Content-Type: application/json" \
      -d '{
        "name": "TaskGuard",
        "repository": {
          "url": "https://github.com/Guard8-ai/TaskGuard",
          "type": "git"
        },
        "language": "en",
        "programming_language": "other"
      }')

    echo "$CREATE_RESPONSE"

    PROJECT_SLUG=$(echo "$CREATE_RESPONSE" | grep -o '"slug":"[^"]*"' | cut -d'"' -f4)

    if [ -z "$PROJECT_SLUG" ]; then
        echo "❌ Failed to create project"
        echo "$CREATE_RESPONSE"
        exit 1
    fi

    echo "✅ Project created with slug: $PROJECT_SLUG"
fi

echo ""
echo "🔨 Triggering build..."

# Step 3: Trigger a build
BUILD_RESPONSE=$(curl -s -X POST \
  "https://readthedocs.org/api/v3/projects/$PROJECT_SLUG/versions/latest/builds/" \
  -H "Authorization: Token $READTHEDOCS_API_KEY" \
  -H "Content-Type: application/json")

echo "$BUILD_RESPONSE"

echo ""
echo "✅ Setup complete!"
echo ""
echo "📖 Your documentation will be available at:"
echo "   https://$PROJECT_SLUG.readthedocs.io"
echo ""
echo "🔍 Monitor build progress at:"
echo "   https://readthedocs.org/projects/$PROJECT_SLUG/builds/"
echo ""
echo "⚙️  Configure webhook integration:"
echo "   https://readthedocs.org/projects/$PROJECT_SLUG/integrations/"
