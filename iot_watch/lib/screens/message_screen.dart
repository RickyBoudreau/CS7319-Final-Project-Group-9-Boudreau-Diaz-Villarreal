import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';

class MessagesAppScreen extends StatelessWidget {
  final SensorRepository repository;

  const MessagesAppScreen({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    const String contactName = 'Main Sensor';
    const String contactInitial = 'M';
    const Color avatarColor = Color(0xFF7986CB);

    return Scaffold(
      backgroundColor: const Color(0xFF333333),
      body: StreamBuilder<WatchUiState>(
        stream: repository.watchStateStream,
        builder: (context, snapshot) {
          final message = snapshot.data?.latestMessage ?? 'No new messages...';
          final isUnread = snapshot.data?.latestMessage != null;

          return Padding(
            padding: const EdgeInsets.all(12.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                GestureDetector(
                  onTap: () => Navigator.of(context).pop(),
                  child: const Padding(
                    padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                    child: Row(
                      children: [
                        Icon(
                          Icons.chevron_left,
                          color: Colors.white70,
                          size: 16,
                        ),
                        Text(
                          'Messages',
                          style: TextStyle(color: Colors.white70, fontSize: 14),
                        ),
                      ],
                    ),
                  ),
                ),
                Expanded(
                  child: Container(
                    width: double.infinity,
                    decoration: BoxDecoration(
                      color: const Color(0xFFF2ECEC),
                      borderRadius: BorderRadius.circular(20),
                    ),
                    padding: const EdgeInsets.all(8.0),
                    child: GestureDetector(
                      onTap: () {
                        if (isUnread) {
                          Navigator.of(context).push(
                            MaterialPageRoute(
                              builder: (context) => SingleMessageDetailScreen(
                                contactName: contactName,
                                initial: contactInitial,
                                avatarColor: avatarColor,
                                fullMessage: message,
                              ),
                            ),
                          );
                        }
                      },
                      child: Container(
                        color: Colors.transparent,
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const CircleAvatar(
                              radius: 16,
                              backgroundColor: avatarColor,
                              child: Text(
                                contactInitial,
                                style: TextStyle(
                                  color: Colors.white,
                                  fontSize: 16,
                                ),
                              ),
                            ),
                            const SizedBox(width: 8),
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  const Text(
                                    contactName,
                                    style: TextStyle(
                                      fontSize: 12,
                                      color: Colors.black87,
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                  const SizedBox(height: 2),
                                  Text(
                                    isUnread ? 'New Message' : message,
                                    style: TextStyle(
                                      fontSize: 10,
                                      color: isUnread
                                          ? Colors.black87
                                          : Colors.black54,
                                    ),
                                    maxLines: 2,
                                    overflow: TextOverflow.ellipsis,
                                  ),
                                ],
                              ),
                            ),
                            if (isUnread)
                              Padding(
                                padding: const EdgeInsets.only(
                                  left: 4.0,
                                  top: 4.0,
                                ),
                                child: Container(
                                  width: 10,
                                  height: 10,
                                  decoration: const BoxDecoration(
                                    color: Colors.red,
                                    shape: BoxShape.circle,
                                  ),
                                ),
                              ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}

class SingleMessageDetailScreen extends StatelessWidget {
  final String contactName;
  final String initial;
  final Color avatarColor;
  final String fullMessage;

  const SingleMessageDetailScreen({
    super.key,
    required this.contactName,
    required this.initial,
    required this.avatarColor,
    required this.fullMessage,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF333333),
      body: GestureDetector(
        onTap: () => Navigator.of(context).pop(),
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Padding(
                padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                child: Row(
                  children: [
                    Icon(Icons.chevron_left, color: Colors.white70, size: 16),
                    Text(
                      'Back',
                      style: TextStyle(color: Colors.white70, fontSize: 14),
                    ),
                  ],
                ),
              ),
              Expanded(
                child: Container(
                  width: double.infinity,
                  decoration: BoxDecoration(
                    color: const Color(0xFFF2ECEC),
                    borderRadius: BorderRadius.circular(20),
                  ),
                  padding: const EdgeInsets.all(12.0),
                  child: SingleChildScrollView(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            CircleAvatar(
                              radius: 12,
                              backgroundColor: avatarColor,
                              child: Text(
                                initial,
                                style: const TextStyle(
                                  color: Colors.white,
                                  fontSize: 12,
                                ),
                              ),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              contactName,
                              style: const TextStyle(
                                fontSize: 14,
                                fontWeight: FontWeight.bold,
                                color: Colors.black87,
                              ),
                            ),
                          ],
                        ),
                        const Divider(color: Colors.grey),
                        const SizedBox(height: 4),
                        Text(
                          fullMessage,
                          style: const TextStyle(
                            fontSize: 12,
                            color: Colors.black87,
                            height: 1.3,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
