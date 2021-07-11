using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Frontend.Models
{
    public class User
    {
        [JsonPropertyName("username")]
        public string Username { get; init; }

        [JsonPropertyName("token")]
        public string Token { get; init; }
    }
}
