package com.example.spring.Controller;

import com.example.spring.Service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.Mapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class UserController {
    @Autowired
    private UserService userService;

    @RequestMapping("/login")
    public Boolean login(@RequestParam("username") String username, @RequestParam("password") String password) {
        return userService.login(username, password);
    }

}
