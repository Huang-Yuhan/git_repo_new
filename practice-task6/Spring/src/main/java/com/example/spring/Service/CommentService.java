package com.example.spring.Service;

import com.example.spring.Entity.CommentEntity;
import com.example.spring.Repository.commentRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class CommentService {
    @Autowired
    private commentRepository commentRepos;

    public void commentMovie(String commentator,String movieName,String commentDetail){
        CommentEntity commentEntity = new CommentEntity();
        commentEntity.setCommentator(commentator);
        commentEntity.setMovieName(movieName);
        commentEntity.setCommentDetail(commentDetail);
        commentRepos.save(commentEntity);
    }
}
