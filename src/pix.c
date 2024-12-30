#include <arpa/inet.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define PORT 1337

int main(int argc, char const *argv[]) {
  int status, valread, client_fd;

  struct sockaddr_in serv_addr;

  const char *domain_name = "wall.c3pixelflut.de";
  struct hostent *host;
  struct sockaddr_in server_addr;

  // Resolve the domain name to an IP address
  host = gethostbyname(domain_name);
  if (host == NULL) {
    fprintf(stderr, "Error: Unable to resolve domain name %s\n", domain_name);
    return EXIT_FAILURE;
  }

  // Create a socket
  int sock = socket(AF_INET, SOCK_STREAM, 0);
  if (sock < 0) {
    perror("Socket creation failed");
    return EXIT_FAILURE;
  }

  // Set up the server address structure
  memset(&server_addr, 0, sizeof(server_addr));
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(PORT);
  memcpy(&server_addr.sin_addr.s_addr, host->h_addr, host->h_length);

  // Connect to the server
  if (connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
    perror("Connection to server failed");
    close(sock);
    return EXIT_FAILURE;
  }

  char buffer[1024] = {0};
  const char *hello = "SIZE\n";
  send(client_fd, hello, strlen(hello), 0);

  fd_set read_set;
  struct timeval timeout;

  timeout.tv_sec = 1; // Time out after a minute
  timeout.tv_usec = 0;

  FD_ZERO(&read_set);
  FD_SET(sock, &read_set);

  int r = select(sock + 1, &read_set, NULL, NULL, &timeout);

  if (r < 0) {
    // Handle the error
    perror("select() failed");
  } else if (r == 0) {
    // Timeout - handle that. You could try waiting again, close the socket...
    perror("select() timed out");
  } else if (r > 0) {
    // The socket is ready for reading - call read() on it.
    valread = read(sock, buffer, 1024 - 1);
    if (valread < 0) {
      perror("read() failed");
    } else {
      buffer[valread] = '\0';
      printf("%s\n", buffer);
    }
  }

  // closing the connected socket
  close(client_fd);
  return 0;
}
