pub mod follow_box;
pub mod search_channels;
/*


  curl -s --request POST \
    --url https://gql.twitch.tv/gql \
    --header 'Accept: application/json' \
    --header 'Accept-Language: en-US' \
    --header 'Client-Id: r8s4dac0uhzifbpu9sjdiwzctle17ff' \
    --header 'Connection: keep-alive' \
    --header 'Content-Type: application/json' \
    --header 'DNT: 1' \
    --header 'Origin: https://m.twitch.tv' \
    --header 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:83.0) Gecko/20100101 Firefox/83.0' \
    --header 'X-Device-Id: SBCCPwRDjdL4rAQ8KYngLGUu9NECqlM6' \
    --data '{"query":"query ChannelProfile_Query($login: String!) {\n  channel: user(login: $login) {\n    ...ChannelInfoCard_user\n    ...ChannelProfileVideos_user\n    id\n    login\n    displayName\n    stream {\n      id\n    }\n    hosting {\n      id\n      __typename\n      login\n      stream {\n        id\n        __typename\n      }\n    }\n  }\n}\n\nfragment ChannelInfoCard_user on User {\n  displayName\n  hosting {\n    id\n  }\n  stream {\n    type\n  }\n}\n\nfragment ChannelProfileVideos_user on User {\n  ...FeaturedContentCard_user\n  login\n  displayName\n  stream {\n     game {\n        name\n      }\n    title\n  }\n  hosting {\n    id\n  }\n}\n\nfragment FeaturedContentCard_user on User {\n  displayName\n  \n  hosting {\n    id\n    login\n    displayName\n    stream {\n      type\n      title\n      game {\n        name\n        id\n      }\n      id\n    }\n  }\n}","variables":{"login":"'"$1"'"},"operationName":"ChannelProfile_Query"}' \


    Original graphQL:
query ChannelProfile_Query($login: String!) {
  channel: user(login: $login) {
    ...ChannelInfoCard_user
    ...ChannelProfileVideos_user
    id
    login
    displayName
    stream {
      id
      broadcaster {
        broadcastSettings {
          title
        }
      }
    }
    hosting {
      id
      __typename
      login
      stream {
        id
        __typename
      }
    }
  }
}

fragment ChannelInfoCard_user on User {
  displayName
  hosting {
    id
  }
  stream {
    type
  }
}

fragment ChannelProfileVideos_user on User {
  ...FeaturedContentCard_user
  login
  displayName
  stream {
    game {
      name
    }
  }

  hosting {
    id
  }
}

fragment FeaturedContentCard_user on User {
  displayName
  hosting {
    id
    login
    displayName
    stream {
      type
      game {
        name
        id
      }
      id
    }
  }
}

VARIABLES:

{
    "login": "CHANNEL ID"
}

*/
