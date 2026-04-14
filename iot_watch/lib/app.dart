import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/screens/dashboard_screen.dart';

class IoTWatchApp extends StatelessWidget {
  final SensorRepository repository;

  const IoTWatchApp({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'IoT Watch UI',
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF333333),
      ),
      builder: (context, child) {
        return Scaffold(
          backgroundColor: Colors.black, 
          body: Center(
            child: ClipRRect(
              borderRadius: BorderRadius.circular(32.0), 
              child: SizedBox(
                width: 200,
                height: 200,
                child: child, 
              ),
            ),
          ),
        );
      },
      // Pass the repository to the dashboard so it can hand it to individual screens
      home: WatchDashboard(repository: repository), 
    );
  }
}