# Test

## GET movie
curl 'http://localhost:3000/movie/1'

## POST movie

curl -H "Content-type: application/json" -d '{
    "id": "2",
    "name": "Second Movie",
    "year": 2001,
    "was_good": true
}' 'http://localhost:3000/movie'
