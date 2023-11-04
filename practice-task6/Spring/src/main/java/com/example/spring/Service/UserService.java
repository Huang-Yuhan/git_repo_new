package com.example.spring.Service;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import com.example.spring.Repository.userRepository;

@Service
public class UserService {
    @Autowired
    private userRepository userRepos;
    public Boolean login(String username, String password) {
        return userRepos.existsByUsernameAndAndPassword(username, password);
    }
}
